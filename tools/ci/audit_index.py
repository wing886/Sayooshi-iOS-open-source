"""Audit the Git index, not untracked game folders. Never prints matched secrets."""
import hashlib, json, re, subprocess, sys
from pathlib import Path
GIT=sys.argv[1] if len(sys.argv)>1 else 'git'
def git(*args):return subprocess.check_output([GIT,*args])
files=[x.decode('utf-8') for x in git('ls-files','-z').split(b'\0') if x]
fixture='crates/siglus_gameexe_compiler/tests/fixtures/Gameexe.ini'
fixture_hash='e0fa92ddc99515c3399711fd601c0886e9aa41baf15e764960cc719141ce53b3'
forbidden_names={'seen.txt','fn.dat','adieu_cn.dat','gameexe.ini','gameexe.dat','scene.pck','hosts.yml','credentials','id_rsa','id_ed25519'}
forbidden_ext={'.sav','.sud','.pdt','.avi','.wav','.koe','.exe','.dll','.ipa','.p12','.pfx','.key','.mobileprovision'}
patterns=[rb'-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----',rb'gh[pousr]_[A-Za-z0-9]{30,}',rb'github_pat_[A-Za-z0-9_]{40,}',rb'AKIA[A-Z0-9]{16}']
errors=[];manifest=[]
# One batch avoids thousands of subprocess launches on Windows.
requests=('\n'.join(':'+name for name in files)+'\n').encode('utf-8')
raw=subprocess.check_output([GIT,'cat-file','--batch'],input=requests)
cursor=0
for name in files:
 end=raw.index(b'\n',cursor);header=raw[cursor:end].split();size=int(header[2]);cursor=end+1
 data=raw[cursor:cursor+size];cursor+=size+1
 p=Path(name);sha=hashlib.sha256(data).hexdigest()
 allowed_fixture=name==fixture and sha==fixture_hash
 if not allowed_fixture and (p.name.lower() in forbidden_names or p.suffix.lower() in forbidden_ext):errors.append([name,'forbidden resource/credential filename'])
 if any(part.lower() in {'game-work','game-original','saves','downloads','reports','logs','gh-config'} for part in p.parts):errors.append([name,'private directory'])
 if data.startswith(b'MZ') or data.startswith(b'PACL'):errors.append([name,'game/archive executable signature'])
 if any(re.search(pattern,data) for pattern in patterns):errors.append([name,'credential pattern'])
 if len(data)>50*1024*1024:errors.append([name,'oversize file requires review'])
 manifest.append({'path':name,'bytes':len(data),'sha256':sha})
result={'passed':not errors,'file_count':len(files),'total_bytes':sum(x['bytes'] for x in manifest),'errors':errors,'manifest':manifest}
print(json.dumps(result,indent=2))
if errors:sys.exit(1)
