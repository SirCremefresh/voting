<script lang="ts">

    let name = '';
    let polls = [];
    let errorMsg = '';
    addEmptyPoll();

    function handleErrors(response) {
        if (!response.ok) {
            throw Error(response.json());
        }
        return response;
    }

    async function postData(url = '', data = {}) {
        return fetch(url, {
            method: 'POST',
            mode: 'cors',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        })
            .then(response => response.json().then(data => ({
                ok: response.ok, data
            })))
    }

    async function saveVoting() {
        errorMsg = '';
        try {
            const response = await postData('http://0.0.0.0:8000/api/votings', {
                name,
                polls
            });
            if (response.ok) {
                const {votingId, adminKey} = response.data;
                location.hash = `#/voting/admin?votingId=${votingId}&adminKey=${adminKey}`
            } else {
                errorMsg = response.data.reason;
            }
        } catch (e) {
            console.log(e)
        }
    }

    function removePoll(index) {
        if (index === 0 && polls.length === 1) {
            addEmptyPoll();
        }
        polls.splice(index, 1);
        polls = [...polls];
    }

    function addEmptyPoll() {
        polls = [
            ...polls,
            {
                name: '',
                description: ''
            }
        ];
    }
</script>

<main>
    <div class="head">
        <h1>A Simple Voting tool</h1>
        <span>A tool i made in my spare time that makes it easy to create votings where the participants are able to vote just on the active poll.</span>
    </div>
    <div class="body">
        <h2>Create a voting</h2>
        <form on:submit|preventDefault={saveVoting}>
            <span class="form-descriptor">Voting:</span>
            <label>
                <span class="form-label">Name: </span>
                <input type="text" name="name" bind:value={name}>
            </label>
            <span class="form-descriptor">
                Polls:
                <button class="button" on:click|preventDefault={addEmptyPoll}>add</button>
            </span>
            <div>
                {#each polls as poll, i}
                    <div class="poll">
                        <div class="flex-row">
                            <div class="flex-grow">
                                <label>
                                    <span class="form-label">Name: </span>
                                    <input type="text" name="poll-name-{i}"
                                           bind:value={poll.name}>
                                </label>
                                <label>
                                    <span class="form-label">Description: </span>
                                    <input type="text" name="poll-description-{i}"
                                           bind:value={poll.description}>
                                </label>
                            </div>
                            <button on:click|preventDefault={() => removePoll(i)} class="remove-poll button-remove">-
                            </button>
                        </div>

                        <hr>
                    </div>
                {/each}
            </div>
            {#if errorMsg !== ''}
                <div class="error-text">{errorMsg}</div>
            {/if}
            <button class="button-submit" type="submit">save voting</button>
        </form>
    </div>
</main>

<style>
    .head {
        background-image: linear-gradient(to bottom right, red, yellow);
        color: white;
        text-align: center;
        /* top left, top right, bottom right, bottom left */
        clip-path: polygon(
                0 0,
                100% 0,
                100% 60%,
                0 100%
        );
        padding: 5rem 20% 7rem;
    }

    .body {
        text-align: center;
        margin-left: 20%;
        margin-right: 20%;
    }

    .body .form-label {
        width: 120px;
        display: inline-block;
        text-align: right;
    }

    .body input {
        width: calc(100% - 126px);
    }

    .form-descriptor {
        display: block;
        font-weight: bold;
        margin-bottom: 5px;
    }

    .poll:last-child hr {
        display: none;
    }

    main {
        margin: 0;
        font: 21px/1.2 system-ui;
    }

    .remove-poll {
        align-self: center;
        padding: 0;
        margin: 10px;
        width: 40px;
        height: 40px;
    }

    .error-text {
        background-color: #e65454;
        border-radius: 5px;
        padding: 5px;
        margin: 10px 30px;
        color: white;
    }
</style>