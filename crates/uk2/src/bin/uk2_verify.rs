use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use uk2::pdt::decode_pdt34;
use uk2::value::InlineStringPart;
use uk2::{
    InstructionArg, InstructionKind, MesProgram, Operand, ResolvedArg, ServiceCommand, Uk2Game,
    Uk2Host, Uk2MapLayout, Uk2Memory, Uk2MusicFile, Uk2MusicKind, Uk2Vm, disassemble_reachable,
};

#[derive(Debug)]
struct BootMenuReached;

#[derive(Debug)]
struct BootInputReached;

impl std::fmt::Display for BootMenuReached {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("UK2 reached its first menu")
    }
}

impl std::error::Error for BootMenuReached {}

impl std::fmt::Display for BootInputReached {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("UK2 reached a text input wait")
    }
}

impl std::error::Error for BootInputReached {}

struct BootProbe {
    game: Uk2Game,
    menu_path: Vec<u16>,
    menus_seen: usize,
    window_count: usize,
    image_count: usize,
    map_count: usize,
    current_map: Option<Uk2MapLayout>,
    map_object_id: Option<u16>,
    music_count: usize,
    command_count: usize,
    last_command: Option<(uk2::TwoOpcode, usize)>,
    last_loaded_mes: String,
    auto_inputs_remaining: usize,
    pending_input_delay: Option<usize>,
}

impl Uk2Host for BootProbe {
    fn load_mes(&mut self, name: &[u8]) -> Result<MesProgram> {
        self.last_loaded_mes = Uk2Game::decode_engine_name(name)?;
        self.game.load_mes_engine_name(name)
    }

    fn poll(&mut self, memory: &mut Uk2Memory) -> Result<()> {
        if let Some(delay) = &mut self.pending_input_delay {
            if *delay == 0 {
                memory.register_mouse_press(3)?;
                self.pending_input_delay = None;
                println!("  simulated left-button press during script input loop");
            } else {
                *delay -= 1;
            }
        }
        Ok(())
    }

    fn command(&mut self, command: &ServiceCommand, memory: &mut Uk2Memory) -> Result<u16> {
        self.command_count += 1;
        self.last_command = Some((command.opcode, command.offset));
        match command.opcode {
            uk2::TwoOpcode::W1 => {
                let Some(ResolvedArg::Values(values)) = command.arguments.first() else {
                    bail!("boot probe: W1 has no window values");
                };
                if values.len() != 7 {
                    bail!("boot probe: W1 has {} values, expected 7", values.len());
                }
                self.window_count += 1;
            }
            uk2::TwoOpcode::W5 => {
                let name = probe_direct_name(command, 0)?;
                self.game.load_pdt34(&name)?;
                self.image_count += 1;
            }
            uk2::TwoOpcode::F0 => {
                let name = probe_direct_name(command, 0)?;
                let map = self.game.load_map(&name)?;
                self.game.render_map(&map, 0, 0)?;
                self.current_map = Some(map);
                self.map_object_id = None;
                self.map_count += 1;
            }
            uk2::TwoOpcode::FA => {
                let Some(ResolvedArg::Values(values)) = command.arguments.first() else {
                    bail!("boot probe: FA has no entity values");
                };
                if values.len() != 5 {
                    bail!("boot probe: FA has {} values, expected five", values.len());
                }
                let mut numbers = [0u16; 5];
                for (number, value) in numbers.iter_mut().zip(values) {
                    *number = value.number()?;
                }
                if let Some(map) = &mut self.current_map {
                    map.apply_fa(numbers);
                }
            }
            uk2::TwoOpcode::F5 => {
                let Some(ResolvedArg::Value(value)) = command.arguments.first() else {
                    bail!("boot probe: F5 has no map object ID");
                };
                self.map_object_id = Some(value.number()?);
                if self.current_map.is_none() {
                    bail!("boot probe: F5 attaches a map before F0 loaded one");
                }
            }
            uk2::TwoOpcode::FB => {
                let Some(ResolvedArg::Values(values)) = command.arguments.first() else {
                    bail!("boot probe: FB has no map cell values");
                };
                if values.len() != 5 {
                    bail!("boot probe: FB has {} values, expected five", values.len());
                }
                let mut numbers = [0u16; 5];
                for (number, value) in numbers.iter_mut().zip(values) {
                    *number = value.number()?;
                }
                if self.map_object_id == Some(numbers[2]) {
                    if let Some(map) = &mut self.current_map {
                        map.apply_fb(numbers[0], numbers[1], numbers[3], numbers[4]);
                    }
                }
            }
            uk2::TwoOpcode::FC | uk2::TwoOpcode::FD => {
                let Some(ResolvedArg::Values(values)) = command.arguments.first() else {
                    bail!("boot probe: {} has no trigger values", command.opcode);
                };
                if values.len() != 2 {
                    bail!(
                        "boot probe: {} has {} values, expected two",
                        command.opcode,
                        values.len()
                    );
                }
                let object_id = values[0].number()?;
                let trigger_id = values[1].number()?;
                if self.map_object_id == Some(object_id) {
                    if let Some(map) = &mut self.current_map {
                        map.set_trigger_enabled(trigger_id, command.opcode == uk2::TwoOpcode::FD);
                    }
                }
            }
            uk2::TwoOpcode::WC => {
                let Some(ResolvedArg::Value(value)) = command.arguments.first() else {
                    bail!("boot probe: WC has no object ID");
                };
                value.number()?;
            }
            uk2::TwoOpcode::FJ => {
                let Some(ResolvedArg::Values(values)) = command.arguments.first() else {
                    bail!("boot probe: FJ has no map viewport values");
                };
                if values.len() != 3 {
                    bail!("boot probe: FJ has {} values, expected three", values.len());
                }
                for value in values {
                    value.number()?;
                }
            }
            uk2::TwoOpcode::FK => {
                // The reference updates map actors and input here. The
                // boot probe has no player input, so one idle tick suffices
                // to reveal the following script command.
            }
            uk2::TwoOpcode::M0 => {
                let name = probe_direct_name(command, 0)?;
                self.game.load_configured_music(&name)?;
                self.music_count += 1;
            }
            uk2::TwoOpcode::M1 => {
                let Some(ResolvedArg::Values(values)) = command.arguments.first() else {
                    bail!("boot probe: M1 has no volume argument");
                };
                values
                    .first()
                    .context("boot probe: M1 has no volume")?
                    .number()?;
            }
            uk2::TwoOpcode::M2 | uk2::TwoOpcode::M4 => {}
            uk2::TwoOpcode::U0 => {
                let Some(ResolvedArg::Value(value)) = command.arguments.first() else {
                    bail!("boot probe: U0 has no cursor value");
                };
                value.number()?;
            }
            uk2::TwoOpcode::U1 => {
                let Some(ResolvedArg::Values(values)) = command.arguments.first() else {
                    bail!("boot probe: U1 has no palette values");
                };
                let bank_index = values
                    .get(1)
                    .context("boot probe: U1 has no palette bank")?
                    .number()?;
                uk2::ColorTable::parse(&self.game.read("color.tbl1")?)?.bank(bank_index)?;
            }
            uk2::TwoOpcode::U3 => {
                let Some(ResolvedArg::Values(values)) = command.arguments.first() else {
                    bail!("boot probe: U3 has no display values");
                };
                if values.len() != 2 {
                    bail!("boot probe: U3 has {} values, expected two", values.len());
                }
                for value in values {
                    value.number()?;
                }
                let name = probe_direct_name(command, 1)?;
                self.game.load_pdt34(&name)?;
                self.image_count += 1;
            }
            uk2::TwoOpcode::UE => {
                let Some(ResolvedArg::Resources(resources)) = command.arguments.get(1) else {
                    bail!("boot probe: UE has no image resource list");
                };
                for resource in resources {
                    let name = Uk2Game::decode_engine_name(resource.resource.string()?)?;
                    self.game.load_pdt34(&name)?;
                    for value in &resource.arguments {
                        value.number()?;
                    }
                    self.image_count += 1;
                }
            }
            uk2::TwoOpcode::FI => {
                let name = probe_direct_name(command, 0)?;
                self.game.load_pdt34(&name)?;
                self.image_count += 1;
            }
            uk2::TwoOpcode::U2 => {
                let Some(ResolvedArg::Value(value)) = command.arguments.first() else {
                    bail!("boot probe: U2 has no object ID");
                };
                value.number()?;
                if self.last_loaded_mes.eq_ignore_ascii_case("s01_002.mes1")
                    && self.auto_inputs_remaining > 0
                    && self.pending_input_delay.is_none()
                {
                    self.pending_input_delay = Some(500);
                    self.auto_inputs_remaining -= 1;
                    println!(
                        "  scheduled left-button press after U2 at MES offset {:#x}; {} remaining",
                        command.offset, self.auto_inputs_remaining
                    );
                }
            }
            uk2::TwoOpcode::W6 => {
                let Some(ResolvedArg::Direct { value, .. }) = command.arguments.get(1) else {
                    bail!("boot probe: W6 has no text operand");
                };
                value.string()?;
            }
            uk2::TwoOpcode::W7 => return Err(BootInputReached.into()),
            uk2::TwoOpcode::W2 => {
                if self.auto_inputs_remaining == 0 {
                    return Err(BootInputReached.into());
                }
                self.auto_inputs_remaining -= 1;
                memory.clear_mouse_events();
            }
            uk2::TwoOpcode::WD => {
                let Some(ResolvedArg::Value(value)) = command.arguments.first() else {
                    bail!("boot probe: WD has no redraw count");
                };
                value.number()?;
            }
            uk2::TwoOpcode::W4 | uk2::TwoOpcode::WA => {}
            uk2::TwoOpcode::W8 => {
                self.menus_seen += 1;
                if let Some(&selection) = self.menu_path.get(self.menus_seen - 1) {
                    let Some(ResolvedArg::Direct { operand, .. }) = command.arguments.first()
                    else {
                        bail!("boot probe: W8 has no result operand");
                    };
                    memory.write_operand(
                        operand,
                        uk2::RuntimeValue::Number {
                            value: selection,
                            width: uk2::NumericWidth::Word,
                        },
                    )?;
                } else {
                    if let Some(ResolvedArg::Values(options)) = command.arguments.get(4) {
                        for (index, option) in options.iter().enumerate() {
                            if let uk2::RuntimeValue::String(bytes) = option {
                                let (label, _, _) = encoding_rs::SHIFT_JIS.decode(bytes);
                                println!("  option {}: {label}", index + 1);
                            }
                        }
                    }
                    return Err(BootMenuReached.into());
                }
            }
            other => bail!(
                "boot probe reached unhandled service {other} at MES offset {:#x}",
                command.offset
            ),
        }
        Ok(0)
    }
}

fn probe_direct_name(command: &ServiceCommand, index: usize) -> Result<String> {
    let Some(ResolvedArg::Direct { value, .. }) = command.arguments.get(index) else {
        bail!("boot probe: missing direct filename argument {index}");
    };
    Uk2Game::decode_engine_name(value.string()?)
}

fn verify_boot_menu(root: &Path, menu_path: Vec<u16>, auto_inputs: usize) -> Result<()> {
    let game = Uk2Game::open(root)?;
    let start = game.start_mes()?;
    let mut probe = BootProbe {
        game,
        menu_path,
        menus_seen: 0,
        window_count: 0,
        image_count: 0,
        map_count: 0,
        current_map: None,
        map_object_id: None,
        music_count: 0,
        command_count: 0,
        last_command: None,
        last_loaded_mes: String::new(),
        auto_inputs_remaining: auto_inputs,
        pending_input_delay: None,
    };
    let mut vm = Uk2Vm::new(start);
    vm.set_instruction_limit(100_000);
    vm.memory_mut().set_timer16_fast_forward(true);
    match vm.run(&mut probe) {
        Err(error) if error.is::<BootInputReached>() => {
            println!(
                "boot reached input {:?} after menu {} and {} services: {} windows, {} images, {} maps, {} music scores loaded",
                probe.last_command, probe.menus_seen, probe.command_count, probe.window_count, probe.image_count, probe.map_count, probe.music_count
            );
            Ok(())
        }
        Err(error) if error.is::<BootMenuReached>() => {
            println!(
                "boot reached menu {}: {} windows, {} images, {} maps, {} music scores loaded",
                probe.menus_seen, probe.window_count, probe.image_count, probe.map_count, probe.music_count
            );
            Ok(())
        }
        Err(error) => Err(error).with_context(|| {
            format!(
                "UK2 boot path failed after {} service calls; last service {:?}; most recent MES {:?}",
                probe.command_count, probe.last_command, probe.last_loaded_mes
            )
        }),
        Ok(status) => bail!("UK2 boot returned status {status} before the first menu"),
    }
}

fn main() -> Result<()> {
    let root = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("usage: uk2_verify <game-directory>"))?;
    let mut files = Vec::new();
    let mut pdt_files = Vec::new();
    let mut map_files = Vec::new();
    let mut music_files = Vec::new();
    for entry in
        std::fs::read_dir(&root).with_context(|| format!("failed to list {}", root.display()))?
    {
        let path = entry?.path();
        if is_mes(&path) {
            files.push(path);
        } else if has_extension(&path, "PDT") {
            pdt_files.push(path);
        } else if has_extension(&path, "MAP") {
            map_files.push(path);
        } else if has_extension(&path, "MMD") {
            music_files.push((path, Uk2MusicKind::Mmd));
        } else if has_extension(&path, "MMM") {
            music_files.push((path, Uk2MusicKind::Mmm));
        }
    }
    files.sort();
    if files.is_empty() {
        bail!("{} contains no .MES files", root.display());
    }

    let mut instructions = 0usize;
    let mut opcode_counts = BTreeMap::<String, usize>::new();
    let mut ue_resource_names = Vec::new();
    for path in &files {
        let program = MesProgram::from_path(path)?;
        let reachable = disassemble_reachable(&program)
            .with_context(|| format!("failed to verify {}", path.display()))?;
        instructions += reachable.len();
        for instruction in reachable.values() {
            if let uk2::disasm::InstructionKind::Command { opcode, .. } = &instruction.kind {
                *opcode_counts.entry(opcode.to_string()).or_default() += 1;
            }
            if let InstructionKind::Command {
                opcode: uk2::TwoOpcode::UE,
                arguments,
            } = &instruction.kind
            {
                for argument in arguments {
                    if let InstructionArg::ResourceList(resources) = argument {
                        for resource in resources {
                            if let Some(name) = inline_resource_name(&resource.resource) {
                                ue_resource_names.push((path.clone(), name));
                            }
                        }
                    }
                }
            }
        }
    }
    let verify_assets = std::env::args_os()
        .nth(2)
        .is_some_and(|arg| arg == "--assets");
    if verify_assets {
        let game = Uk2Game::open(&root)?;
        for (mes_path, name) in &ue_resource_names {
            let engine_name = Uk2Game::decode_engine_name(name)?;
            let bytes = game.read(&engine_name).with_context(|| {
                format!(
                    "{} references missing UE resource {engine_name:?}",
                    mes_path.display()
                )
            })?;
            let header = uk2::pdt::Pdt34Header::parse(&bytes).with_context(|| {
                format!(
                    "{} UE resource {engine_name:?} is not valid PDT34",
                    mes_path.display()
                )
            })?;
            header.decode_image(&bytes).with_context(|| {
                format!(
                    "{} UE resource {engine_name:?} cannot be decoded",
                    mes_path.display()
                )
            })?;
        }
        pdt_files.sort();
        for path in &pdt_files {
            let bytes = std::fs::read(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            decode_pdt34(&bytes).with_context(|| format!("failed to verify {}", path.display()))?;
        }
        map_files.sort();
        for path in &map_files {
            let map = Uk2MapLayout::from_path(path)
                .with_context(|| format!("failed to verify {}", path.display()))?;
            for resource in [&map.base, &map.overlay] {
                if !game.contains_resource(resource) {
                    bail!(
                        "{} references missing map resource {resource:?}",
                        path.display()
                    );
                }
            }
            game.render_map(&map, 0, 0)
                .with_context(|| format!("failed to render {}", path.display()))?;
        }
        music_files.sort_by(|(left, _), (right, _)| left.cmp(right));
        for (path, kind) in &music_files {
            let music = Uk2MusicFile::from_path(*kind, path)
                .with_context(|| format!("failed to verify {}", path.display()))?;
            if *kind == Uk2MusicKind::Mmd {
                for track in 0..music.mmd_tracks().len() {
                    music.decode_mmd_track(track).with_context(|| {
                        format!("failed to verify MMD track {track} in {}", path.display())
                    })?;
                }
            }
        }
        println!(
            "verified {} PDT, {} MAP, {} music files, and {} literal UE resources",
            pdt_files.len(),
            map_files.len(),
            music_files.len(),
            ue_resource_names.len()
        );
    }
    println!(
        "verified {} MES files, {} reachable instructions",
        files.len(),
        instructions
    );
    if std::env::args_os().any(|arg| arg == "--stats") {
        let mut counts: Vec<_> = opcode_counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        println!("reachable command frequencies:");
        for (opcode, count) in counts {
            println!("{opcode}: {count}");
        }
    }
    if std::env::args_os().any(|arg| arg == "--boot-menu") {
        verify_boot_menu(&root, Vec::new(), 0)?;
    }
    if let Some(count) =
        std::env::args().find_map(|arg| arg.strip_prefix("--boot-choices=").map(str::to_owned))
    {
        let auto_menus = count
            .parse::<usize>()
            .context("invalid --boot-choices count")?;
        if auto_menus > 8 {
            bail!("--boot-choices may not exceed 8");
        }
        verify_boot_menu(&root, vec![1; auto_menus], 0)?;
    }
    if let Some(path) =
        std::env::args().find_map(|arg| arg.strip_prefix("--boot-path=").map(str::to_owned))
    {
        let menu_path = path
            .split(',')
            .map(str::parse::<u16>)
            .collect::<std::result::Result<Vec<_>, _>>()
            .context("invalid --boot-path")?;
        if menu_path.len() > 8 || menu_path.iter().any(|selection| *selection > 20) {
            bail!("--boot-path may contain at most 8 menu indices in 0..20");
        }
        let auto_inputs = std::env::args()
            .find_map(|arg| arg.strip_prefix("--boot-inputs=").map(str::to_owned))
            .map(|count| {
                count
                    .parse::<usize>()
                    .context("invalid --boot-inputs count")
            })
            .transpose()?
            .unwrap_or(0);
        if auto_inputs > 100 {
            bail!("--boot-inputs may not exceed 100");
        }
        verify_boot_menu(&root, menu_path, auto_inputs)?;
    }
    Ok(())
}

fn inline_resource_name(expression: &uk2::value::Expression) -> Option<Vec<u8>> {
    if expression.terms.len() != 1 {
        return None;
    }
    let Operand::InlineString(parts) = &expression.terms[0].operand else {
        return None;
    };
    let mut name = Vec::new();
    for part in parts {
        match part {
            InlineStringPart::Literal(bytes) => name.extend_from_slice(bytes),
            InlineStringPart::Table5Slot(_) | InlineStringPart::Table6Slot(_) => return None,
        }
    }
    Some(name)
}

fn has_extension(path: &Path, wanted: &str) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case(wanted))
}

fn is_mes(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("mes"))
}
