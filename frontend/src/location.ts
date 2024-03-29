export function parseQuery() {
    const keyValueList = window.location
        .hash
        .trim()
        .replace(/^.*\?/, "")
        .split("&")
        .map(queryVar => ({
            key: queryVar.replace(/=.*/, ""),
            value: queryVar.replace(/^.*=/, "")
        }));

    const queryMap = new Map();
    for (let {key, value} of keyValueList) {
        queryMap.set(key, value)
    }
    return queryMap;
}

export function getHashPath() {
    return window.location
        .hash
        .trim()
        .replace(/^(#\/)/, "")
        .replace(/\?.*/, "");
}