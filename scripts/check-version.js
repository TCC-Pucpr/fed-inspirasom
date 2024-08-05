const tauriConfigFile = '../src-tauri/tauri.conf.json';

run();

function run() {
    const regex = /\d*\.\d*\.\d*/;
    let tauriConfig = require(tauriConfigFile);
    const tauriVersion = tauriConfig.package.version;  

    const matchesLatestVersion = process.argv[2].match(regex);
    if(matchesLatestVersion.length === 0){
        throw ('Something unexpected happened, couldn\'t retrieve version information');
    }
    const latestVersion = matchesLatestVersion[0];
    
    if(process.argv[2] === '0.0.0') {
        latestVersion = process.argv[2];
    } else if(!latestVersion){
        throw (`Something went very very VERY wrong, and we got no versions from the latest release. Got tag [${process.argv[2]}], with version [${latestVersion}].\nThe correct format should be [anything really]v*.*.*`)
    }
    
    const exec = require('child_process').exec;
    exec(`npm version`, (err, stdout) => {
        if(err) throw ('Something unexpected happened, couldn\'t retrieve version information');
        const matchesPackageVer = stdout.match(regex);

        if(matchesPackageVer.length === 0){
            throw ('Something unexpected happened, couldn\'t retrieve version information');
        }
        const packageVersion = matchesPackageVer[0];

        console.log(`.latest: ${latestVersion}`);
        console.log(`package: ${packageVersion}`);

        if(packageVersion !== tauriVersion){
            throw ('tauri and package version mismatch!\nPlease run \'npm run ver p\' to fix this problem')
        }
        if(tauriVersion === latestVersion){
            throw ('The new release must have a different version number than the last one!\nPlease run \'npm run ver p\' to fix this problem')
        }

        const latestVersionSplit = latestVersion.split('.');
        const packageSplit = packageVersion.split('.');

        for(let i = 0; i < latestVersionSplit.length; i++){
            if(packageSplit[i] < latestVersionSplit[i]){
                throwVersionError();
                return;
            }
        }
    });
}

function throwVersionError() {
    throw ('The current package version is lower than the latest!\n Please set a higher version with \'npm run ver s [version]\'')
}