let fs = require("fs");
let path = require("path");
let binFile = "actix-tom";
let inputDir = ".";
let outputDir = "./target/tar";
let defaultDir = "./target/release";
let linuxDir = "./target/x86_64-unknown-linux-musl/release";

function copyRecursiveSync(src, dest) {
  let exists = fs.existsSync(src);
  let stats = exists && fs.statSync(src);
  let isDirectory = exists && stats.isDirectory();
  if (isDirectory) {
    fs.mkdirSync(dest);
    fs.readdirSync(src).forEach(function (childItemName) {
      copyRecursiveSync(
        path.join(src, childItemName),
        path.join(dest, childItemName)
      );
    });
  } else {
    fs.copyFileSync(src, dest);
  }
}

function copyResource() {
  let file = ["static", "templates", "config.toml"];
  file.forEach(async (f) => {
    copyRecursiveSync(`${inputDir}/${f}`, `${outputDir}/${f}`);
  });
}

function copyLinux() {
  copyRecursiveSync(`${linuxDir}/${binFile}`, `${outputDir}/${binFile}`);
}

function copyDefault() {
  copyRecursiveSync(`${defaultDir}/${binFile}`, `${outputDir}/${binFile}`);
}

async function checkDir(reaPath) {
  const absPath = path.resolve(__dirname, reaPath);
  try {
    await fs.promises.stat(absPath);
  } catch (e) {
    await fs.promises.mkdir(absPath, { recursive: true });
  }
}

async function deleteFolder(path) {
  let files = [];
  if (fs.existsSync(path)) {
    files = fs.readdirSync(path);
    files.forEach(function (file, index) {
      let curPath = path + "/" + file;
      if (fs.statSync(curPath).isDirectory()) {
        // recurse
        deleteFolder(curPath);
      } else {
        // delete file
        fs.unlinkSync(curPath);
      }
    });
    fs.rmdirSync(path);
  }
}

async function main(argv) {
  let type = argv[0];
  await deleteFolder(outputDir);
  await checkDir(outputDir);
  switch (type) {
    case "linux":
      copyLinux();
      break;
    default:
      copyDefault();
      break;
  }
  copyResource();
}

main(process.argv.slice(2));
