const fs = require('fs');
const { dirname } = require('path');
const tauriConfigFile = '../src-tauri/tauri.conf.json';

run();

function run() {

    let tauriConfig = require(tauriConfigFile);
    let operation = "";

    switch(process.argv[2]) {
        case "p":
        case "patch":
            operation = "patch"
            break;
        case "m":
        case "minor":
            operation = "minor"
            break;
        case "M":
        case "major":
            operation = "major"
            break;
        case "s":
        case "set":
            if(process.argv<4){
                help();
                return;
            } else {
                operation = process.argv[3].replace('/\D/', '');
            }
            break;
        default:
            help();
            return;
    }

    const appDir = dirname(require.main.filename);
    const exec = require('child_process').exec;
    
    exec(`npm version ${operation} --no-git-tag-version`, (err, stdout) => {
        try{
            const currentVer = stdout.slice(1).replace(/\n/, '');
            tauriConfig.package.version = currentVer;
            fs.writeFileSync(`${appDir}/${tauriConfigFile}`, JSON.stringify(tauriConfig, null, 2));
            return;
        } catch (error) {
            console.log(error);
            if(process.argv[2] === "set" || process.argv[2] === "s"){
                console.error('invalid version number! The correct format is [major].[minor].[patch]');
                console.error('ex. > npm run version set 1.2.3');
                console.error('Please change it and try again');
            } else {
                console.error('something went wrong!');
            }
        }
    });

}
        
function help(){
    console.log("this script auto updates tauri to match the npm project version");
    console.log("");
    console.log("p patch                ups patch version");
    console.log("m minor                ups minor version");
    console.log("M major                ups major version");
    console.log("s set [version]        sets version to specified number");
    console.log("");
}