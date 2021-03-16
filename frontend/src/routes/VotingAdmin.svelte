<script lang="ts">
    import {parseQuery} from "../location";
    import CopyClipBoard from "../CopyClipBoard.svelte";
    import {getData, postData} from "../api";
    import {onMount, onDestroy} from 'svelte';

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
        const response = await getData(`${process.env.apiUrl}/votings/${votingId}`, adminKey);
        if (response.ok) {
            voting = response.data;
        } else {
            errorMsg = response.data.reason;
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
        const response = await postData(`${process.env.apiUrl}/votings/${votingId}/voters`, {username: voterUsername}, adminKey)
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
        const response = await postData(`${process.env.apiUrl}/votings/${votingId}/polls/active`, {pollIndex: index}, adminKey)
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
            const updateInterval = setInterval(async () => {
                await loadVoting();
            }, 1000);

            onDestroy(() => {
                if (updateInterval) {
                    clearInterval(updateInterval);
                }
            })
        });
    }

</script>


<main>
    <div class="head">
        <h1>Voting:</h1>
        <h2>{voting.name}</h2>
        <span>{voting.voterCount} voters</span>
    </div>
    <div class="body">
        {#if errorMsg !== ''}
            <div class="error-text">{errorMsg}. Please reload and try again</div>
        {/if}
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
        <h3>Polls</h3>
        <div class="polls">
            {#each voting.polls as poll, i}
                <div class="poll flex-row flex-align-center" class:active={voting.activePollIndex === i}>
                    <div class="flex-grow">
                        <h4 class="title">{poll.name}</h4>
                        <span class="description">{poll.description}</span>
                        <span class="status">
                            status:
                            <strong>
                            {#if poll.status === 'NOT_VOTED'}
                                not voted
                            {:else if poll.status === 'ACCEPTED'}
                               accepted
                            {:else if poll.status === 'DECLINED'}
                                declined
                            {:else}
                                draw
                            {/if}
                            </strong>
                        </span>
                        accepted: <strong>{poll.votesAccept}</strong>
                        declined: <strong>{poll.votesDecline}</strong>
                        abstain: <strong>{poll.votesAbstain}</strong>
                        total: <strong>{poll.votesTotal}</strong>
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

    @media (max-width: 600px) {
        .body {
            margin-left: 0;
            margin-right: 0;
        }
    }

    .poll {
        text-align: left;
        background-color: #f6f6f6;
        margin: 10px;
        padding: 10px;
        border-radius: 4px;
    }

    .poll.active {
        background-color: #def3de;
    }

    .poll .title {
        display: block;
        font-weight: bold;
        margin-top: 4px;
        margin-bottom: 4px;
    }

    .poll .description {
        display: block;
    }

    .poll .status {
        display: block;
    }

</style>