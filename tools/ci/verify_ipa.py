import argparse,zipfile,plistlib,struct,json,hashlib
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('ipa');p.add_argument('--commit',required=True);a=p.parse_args();f=Path(a.ipa)
with zipfile.ZipFile(f) as z:
 assert z.testzip() is None,'ZIP CRC failure'
 infos=[n for n in z.namelist() if n.startswith('Payload/') and n.count('/')==2 and n.endswith('/Info.plist')];assert len(infos)==1
 info_path=infos[0];root=info_path.rsplit('/',1)[0];info=plistlib.loads(z.read(info_path));exe=z.read(root+'/'+info['CFBundleExecutable'])
 assert exe[:4]==b'\xcf\xfa\xed\xfe','expected thin 64-bit Mach-O'
 cpu=struct.unpack_from('<I',exe,4)[0];assert cpu==0x0100000c,'expected arm64'
 version=tuple(map(int,info['MinimumOSVersion'].split('.')));assert version<=(17,0),'minimum OS exceeds iOS 17'
 names=[n for n in z.namelist() if n.startswith(root+'/') and n.endswith('/BuildProvenance.json')];assert len(names)==1
 provenance=json.loads(z.read(names[0]));assert provenance['source_commit']==a.commit,'wrong source commit'
 assert 'raw-glyph-history' in provenance['fixes']
 assert any(n.endswith('/ThirdPartyNotices.txt') for n in z.namelist()),'license notices missing'
 assert not any(n.endswith('/embedded.mobileprovision') for n in z.namelist()),'unexpected provisioning profile'
 result={'sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'bytes':f.stat().st_size,'architecture':'arm64','MinimumOSVersion':info['MinimumOSVersion'],'CFBundleIdentifier':info['CFBundleIdentifier'],'CFBundleShortVersionString':info['CFBundleShortVersionString'],'CFBundleVersion':info['CFBundleVersion'],'UIFileSharingEnabled':info.get('UIFileSharingEnabled'),'source_provenance':provenance,'structure_verified':True,'iPhone_runtime_test':'not yet performed'}
 print(json.dumps(result,indent=2))
