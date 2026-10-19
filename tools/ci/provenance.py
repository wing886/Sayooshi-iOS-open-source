import hashlib,json,os,subprocess
from pathlib import Path
files=['crates/avg32/src/nls.rs','crates/avg32/src/meswin.rs','crates/avg32/src/menu.rs','crates/avg32/src/scenario.rs','crates/avg32/src/system.rs','crates/avg32/src/ini.rs','crates/avg32/src/font.rs']
p=Path('platform/ios/SiglusLauncher/SiglusLauncher/Resources/BuildProvenance.json')
p.write_text(json.dumps({'upstream':'5f358ef1360b0439e92aed2b749e8f25d8721954','source_commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'run_id':os.environ.get('GITHUB_RUN_ID'),'fixes':['raw-SJIS-novel-font','raw-glyph-history','history-opcodes-03-04-05','independent-title-encoding'],'source_sha256':{f:hashlib.sha256(Path(f).read_bytes()).hexdigest() for f in files}},indent=2))

spec=Path("platform/ios/SiglusLauncher/project.yml")
spec.write_text(spec.read_text().replace('CURRENT_PROJECT_VERSION: "1"', 'CURRENT_PROJECT_VERSION: "'+os.environ.get("GITHUB_RUN_NUMBER","1")+'"'))
