module.exports.readVersion = function (contents) {
    let result = contents.match("version = \"(\\d+\.\\d+\.\\d+)");
    let version = result[1];
    return version;
};

module.exports.writeVersion = function (contents, version) {
    contents = contents.replace(/version = \"\d+\.\d+\.\d+/, "version = \"" + version);
    return contents;
};