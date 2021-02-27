<script lang="ts">
    import {parseQuery} from "../location";
    import CopyClipBoard from "../CopyClipBoard.svelte";
    import {getData, postData, putData} from "../api";
    import {onMount} from 'svelte';

    let voting = {
        votingId: "",
        name: "",
        voterCount: 0,
        polls: [],
        activePollIndex: null
    }

    let voterUsername = '';
    let voterErrorMsg = '';
    let voterUrl = '';

    let errorMsg = '';

    const parsedQuery = parseQuery();
    const votingId = parsedQuery.get('votingId');
    const adminKey = parsedQuery.get('adminKey');

    async function loadVoting() {
        const response = await getData(`http://0.0.0.0:8000/api/votings/${votingId}`, adminKey);
        if (response.ok) {
            voting = response.data;
        } else {
            alert('could not load data. reload page');
        }
    }

    function copyVoterUrl() {
        const app = new CopyClipBoard({
            target: document.getElementById('clipboard'),
            props: {text: voterUrl},
        });
        app.$destroy();
    }

    async function addVoter() {
        voterErrorMsg = '';
        voterUrl = '';
        const response = await postData(`http://0.0.0.0:8000/api/votings/${votingId}/voters`, {username: voterUsername}, adminKey)
        if (response.ok) {
            const {votingId, voterKey} = response.data;
            voterUrl = `${location.origin}/#/voting/voter?votingId=${votingId}&voterKey=${voterKey}&username=${voterUsername}`;
            voterUsername = '';
            loadVoting();
        } else {
            voterErrorMsg = response.data.reason;
        }
    }

    async function setActivePoll(index) {
        const response = await postData(`http://0.0.0.0:8000/api/votings/${votingId}/polls/active`, {pollIndex: index}, adminKey)
        if (response.ok) {
            loadVoting();
        } else {
            errorMsg = response.data.reason;
        }
    }

    if (votingId === undefined || adminKey === undefined) {
        location.hash = '#/not-found'
    } else {
        onMount(async () => {
            await loadVoting();
        });
    }

</script>


<main>
    <div class="head">
        <h1>Voting:</h1>
        <h2>{voting.name}</h2>
    </div>
    <div class="body">
        {#if errorMsg !== ''}
            <div class="error-text">{errorMsg}</div>
        {/if}        <h3>Add Voter</h3>
        <form on:submit|preventDefault={addVoter}>
            <div class="flex-row flex-align-center">
                <label class="" for="username">Username: </label>
                <input class="flex-grow" type="text" name="username" id="username" bind:value={voterUsername}>
                <button class="button-submit">add</button>
            </div>
        </form>
        {#if voterUrl !== ''}
            <div class="flex-row">
                <code class="flex-grow">{voterUrl}</code>
                <button class="button" on:click={copyVoterUrl}>copy</button>
            </div>
        {/if}
        {#if voterErrorMsg !== ''}
            <div class="error-text">{voterErrorMsg}</div>
        {/if}
        <h3>Polls</h3>
        <div class="polls">
            {#each voting.polls as poll, i}
                <div class="poll flex-row flex-align-center">
                    <div class="flex-grow">
                        <span class="title">{poll.name}</span>
                        <span class="description">{poll.description}</span>

                        {poll.votesAbstain}
                        {poll.votesAccept}
                        {poll.votesDecline}
                        {poll.votesTotal}
                        is active: {voting.activePollIndex === i}
                        {i}
                    </div>
                    {#if voting.activePollIndex === i}
                        <button class="button-remove" on:click={() => setActivePoll(null)}>deactivate</button>
                    {:else}
                        <button class="button" on:click={() => setActivePoll(i)}>activate</button>
                    {/if}
                </div>
            {/each}
        </div>
    </div>
    <div id="clipboard"></div>
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
                100% 80%,
                0 100%
        );
        padding: 1rem 20% 2rem;
    }

    .body {
        text-align: center;
        margin-left: 20%;
        margin-right: 20%;
    }

    .poll {
        text-align: left;
        background-color: #f6f6f6;
        margin: 10px;
        padding: 10px;
        border-radius: 4px;
    }

    .poll .title {
        display: block;
        font-weight: bold;
    }

    .poll .description {
        display: block;
    }

    @media (max-width: 600px) {
        .body {
            margin-left: 0;
            margin-right: 0;
        }
    }

</style>