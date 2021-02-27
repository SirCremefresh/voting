<script lang="ts">
    import {parseQuery} from "../location";
    import CopyClipBoard from "../CopyClipBoard.svelte";
    import {getData, postData} from "../api";
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
            props: {text:voterUrl},
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
        } else {
            voterErrorMsg = response.data.reason;
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
        <h3>Add Voter</h3>
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
</style>