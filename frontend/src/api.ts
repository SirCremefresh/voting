export async function postData(url = '', data = {}, token = undefined) {
    return fetch(url, {
        method: 'POST',
        mode: 'cors',
        headers: {
            'Content-Type': 'application/json',
            ...(token === undefined ? {} : {Authorization: token})
        },
        body: JSON.stringify(data)
    })
        .then(response =>
            response
                .json()
                .then(data => ({
                    ok: response.ok, data
                }))
        )
}

export async function getData(url = '', token = undefined) {
    return fetch(url, {
        method: 'GET',
        mode: 'cors',
        headers: {
            'Content-Type': 'application/json',
            ...(token === undefined ? {} : {Authorization: token})
        },
    })
        .then(response =>
            response
                .json()
                .then(data => ({
                    ok: response.ok, data
                }))
        )
}