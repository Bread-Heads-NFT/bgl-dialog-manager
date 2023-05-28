const path = require("path");
const { generateIdl } = require("@metaplex-foundation/shank-js");

const idlDir = path.join(__dirname, "..", "idls");
const binaryInstallDir = path.join(__dirname, "..", ".crates");
const programDir = path.join(__dirname, "..", "programs");

generateIdl({
  generator: "shank",
  programName: "bgl_dialog_manager",
  programId: "D1ALGLQpQ1mYs2QpNfuEVposQp5fwvJAep3y2gUgLo6",
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "bgl-dialog-manager"),
});
