# SPDX-License-Identifier: MPL-2.0
"""Review all reachable Git history. Findings never include matched secret values."""
import argparse, hashlib, json, re, subprocess, sys
from pathlib import PurePosixPath

def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--git',default='git')
    args=parser.parse_args()
    def git(*cmd): return subprocess.check_output([args.git,*cmd])
    commits=git('rev-list','--all').decode().splitlines()
    paths={}
    for commit in commits:
        for entry in git('ls-tree','-rz',commit).split(b'\0'):
            if not entry: continue
            meta,name=entry.split(b'\t',1)
            mode,kind,oid=meta.split()
            if kind==b'blob': paths.setdefault(oid.decode(),set()).add(name.decode('utf-8'))
    ids=list(paths)
    raw=b'' if not ids else subprocess.check_output([args.git,'cat-file','--batch'],input=('\n'.join(ids)+'\n').encode())
    cursor=0; errors=[]; review=[]; count=0
    forbidden_names={'seen.txt','fn.dat','adieu_cn.dat','gameexe.ini','gameexe.dat','scene.pck','save.ini','read.sav','hosts.yml','credentials','id_rsa','id_ed25519'}
    forbidden_ext={'.sav','.sud','.cad','.pdt','.avi','.wav','.koe','.exe','.dll','.ipa','.p12','.pfx','.key','.pem','.mobileprovision','.ips','.dmp'}
    patterns={
        'private_key':rb'-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----',
        'access_token':rb'gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{40,}|AKIA[A-Z0-9]{16}',
        'personal_path':rb'[A-Za-z]:[\\/](?:Users[\\/][^\\/\r\n]+|Sayooshi_iOS_Setup[\\/])',
        'device_identifier':rb'0000[0-9A-Fa-f]{4}-[0-9A-Fa-f]{16}',
        'personal_account':rb'[A-Za-z0-9._%+-]+@(?:163\.com|qq\.com|icloud\.com)',
    }
    for oid in ids:
        end=raw.index(b'\n',cursor); header=raw[cursor:end].split(); size=int(header[2]); cursor=end+1
        data=raw[cursor:cursor+size]; cursor+=size+1; count+=1
        digest=hashlib.sha256(data).hexdigest()
        for name in sorted(paths[oid]):
            p=PurePosixPath(name)
            fixture=name=='crates/siglus_gameexe_compiler/tests/fixtures/Gameexe.ini' and digest=='e0fa92ddc99515c3399711fd601c0886e9aa41baf15e764960cc719141ce53b3'
            if not fixture and (p.name.lower() in forbidden_names or p.suffix.lower() in forbidden_ext): errors.append({'path':name,'kind':'private_resource_or_credential_file','object':oid})
            if data.startswith((b'MZ',b'PACL')):errors.append({'path':name,'kind':'executable_or_game_archive','object':oid})
            if p.suffix.lower()=='.cfx':review.append({'path':name,'kind':'binary_sample_provenance_unresolved','object':oid})
            for label,pattern in patterns.items():
                if re.search(pattern,data): errors.append({'path':name,'kind':label,'object':oid})
    # Commit messages/identities are also disclosed when history becomes public.
    metadata=git('log','--all','--format=%B%n%an <%ae>%n%cn <%ce>')
    for label,pattern in patterns.items():
        if re.search(pattern,metadata):errors.append({'path':'<commit-metadata>','kind':label})
    report={'passed':not errors and not review,'commit_count':len(commits),'unique_blobs':count,'errors':errors,'review_required':review,'scope':'All local refs reachable commits; no reflogs, remote dangling objects, hosted issue contents or CI log audit. Pattern scan is not proof of absence.'}
    print(json.dumps(report,indent=2))
    return 1 if errors or review else 0
if __name__=='__main__':sys.exit(main())