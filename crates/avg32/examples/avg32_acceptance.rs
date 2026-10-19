//! Local acceptance driver. Reads a supplied installation; never embeds game data.
use std::{path::Path, io::Write};
use anyhow::{Result,ensure,bail};
use avg32::{Avg32Engine,EngineOptions,Nls,Key};
fn shot(e:&mut Avg32Engine,out:&Path,name:&str)->Result<()> {
 let pixels=e.frame_rgba();let mut f=std::fs::File::create(out.join(format!("{name}.ppm")))?;write!(f,"P6\n640 480\n255\n")?;
 for p in pixels.chunks_exact(4){f.write_all(&p[..3])?;}Ok(())
}
fn tick(e:&mut Avg32Engine)->Result<()> {e.tick();e.advance_clock(17);let w=e.take_warnings();ensure!(w.is_empty(),"engine warnings: {:?}",w);Ok(())}
fn choices(e:&Avg32Engine)->usize {let s=format!("{:?}",e.system().sel);if s.contains("drawn: true") {s.matches("SelectItem {").count()}else{0}}
fn main()->Result<()> {
 let a:Vec<String>=std::env::args().collect();ensure!(a.len()>=4,"usage: avg32_acceptance ROOT journey|reload|media OUT");let root=Path::new(&a[1]);let out=Path::new(&a[3]);std::fs::create_dir_all(out)?;
 if a[2]=="skip" {
  let mut positions=Vec::new();
  for held in [false,true] {
   let mut e=Avg32Engine::open(root,EngineOptions{audio:false,virtual_clock:true,persist:true,nls:Nls::Sjis})?;
   ensure!(e.system().slots[0].valid,"need acceptance save first");e.load(0);
   for _ in 0..200 {tick(&mut e)?;}
   e.set_skip(held);for _ in 0..300 {tick(&mut e)?;}
   e.set_skip(false);positions.push((e.scenario().seen,e.scenario().pos,e.system().mes.ptr));e.shutdown();
  }
  ensure!(positions[0]!=positions[1],"hold-to-skip made no difference");
  println!("HOLD_TO_SKIP_OK normal={:?} held={:?}",positions[0],positions[1]);return Ok(());
 }
 if a[2]=="media" {
  let mut wavs=0;let mut movies=0;let mut frames=0;
  for dir in ["BGM","DAT"] {for p in std::fs::read_dir(root.join(dir))? {let p=p?.path();if p.extension().is_some_and(|x|x.eq_ignore_ascii_case("wav")){avg32::sound::wav_clip(&std::fs::read(&p)?,false)?;wavs+=1;}}}
  for p in std::fs::read_dir(root.join("MOV"))? {let p=p?.path();if !p.extension().is_some_and(|x|x.eq_ignore_ascii_case("avi")){continue;}
   let mut movie=avg32::movie::AviMovie::parse(&std::fs::read(&p)?)?;frames+=movie.video_frames.len();movie.frame_interval=std::time::Duration::ZERO;
   let mut gfx=avg32::pdtmgr::PdtManager::new();if let Some(mut player)=avg32::movie::MoviePlayer::start(&movie,false,avg32::pdtmgr::Rect::full(),&mut gfx)? {for _ in 0..movie.video_frames.len()+1 {player.tick(&mut gfx)?;}ensure!(!player.active(),"movie did not finish: {}",p.display());}movies+=1;
  }
  println!("MEDIA_DECODE_OK wavs={wavs} movies={movies} video_frames={frames}; audible output not verified");return Ok(());
 }
 let mut e=Avg32Engine::open(root,EngineOptions{audio:false,virtual_clock:true,persist:true,nls:Nls::Sjis})?;
 println!("revision={}",e.system().version);let w=e.take_warnings();ensure!(w.is_empty(),"startup warnings: {:?}",w);
 if a[2]=="reload" {
  let expected=std::fs::read_to_string(out.join("saved-position.txt"))?;let v:Vec<i32>=expected.split_whitespace().map(|s|s.parse().unwrap()).collect();ensure!(e.system().slots[0].valid,"saved slot not found in fresh process");
  e.load(0);for _ in 0..200 {tick(&mut e)?;}ensure!(e.running()&&e.scenario().seen==v[0],"reload scene differs");ensure!(e.scenario().last_save_seen==v[0]&&e.scenario().last_save_pos==v[1],"replayed save point differs");shot(&mut e,out,"reloaded")?;
  let before=(e.scenario().seen,e.scenario().pos,e.system().mes.ptr);
  for f in 0..600 {if f%45==0{e.key_down(Key::Enter);}tick(&mut e)?;}
  ensure!((e.scenario().seen,e.scenario().pos,e.system().mes.ptr)!=before,"cannot advance after reload");println!("FRESH_PROCESS_LOAD_OK saved_scene={} saved_pos={} current_title={}",v[0],v[1],e.window_title());e.shutdown();return Ok(());
 }
 ensure!(a[2]=="journey","unknown mode");e.jump_to_scene(10);let mut previous=-1;let mut selected=false;let mut after=0;
 for f in 0..60000 {
  if e.scenario().seen!=previous {previous=e.scenario().seen;println!("frame={f} scene={previous} title={}",e.window_title());}
  let count=choices(&e);
  if e.scenario().seen==21 && f>36000 && !selected {
   shot(&mut e,out,"branch-before")?;
   println!("GRAPHICAL_BRANCH scene=21 selected=first-location x=130 y=164");
   e.mouse_move(130,164);e.mouse_up(false);selected=true;
  }else if count>0 && !selected && e.scenario().seen!=11 {
   shot(&mut e,out,"branch-before")?;println!("BRANCH_OPTIONS count={count} scene={} pos={}",e.scenario().seen,e.scenario().pos);
   let ini=e.system().ini();let (x,y)=(ini.win_x+40,ini.win_y+ini.font_y/2);e.mouse_move(x,y);e.mouse_up(false);e.key_down(Key::Enter);selected=true;
  }else if f%45==0 {
   if e.scenario().seen==11 {e.mouse_move(525,180);e.mouse_up(false);}else{e.key_down(Key::Enter);}
  }
  tick(&mut e)?;ensure!(e.running(),"game stopped at scene {}",e.scenario().seen);
  if selected && choices(&e)==0 {after+=1;}
  if selected && e.scenario().seen!=21 && after>=500 && e.scenario().last_save_seen>0 && !e.system().mes.is_empty() {
   shot(&mut e,out,"branch-after")?;e.save(0);tick(&mut e)?;ensure!(e.system().slots[0].valid,"save failed");
   std::fs::write(out.join("saved-position.txt"),format!("{} {}",e.scenario().last_save_seen,e.scenario().last_save_pos))?;
   e.wheel(true);ensure!(e.system().backlog_view.is_some(),"backlog did not open");shot(&mut e,out,"backlog")?;e.key_down(Key::Escape);
   println!("SAVE_AND_BRANCH_OK scene={} pos={} title={} backlog_pages={}",e.scenario().last_save_seen,e.scenario().last_save_pos,e.window_title(),e.system().backlog.len());e.shutdown();return Ok(());
  }
 }
 shot(&mut e,out,"pending-branch")?;
 e.wheel(true);shot(&mut e,out,"backlog-before-fix")?;
 bail!("No completed branch/save within bounded run")
}
