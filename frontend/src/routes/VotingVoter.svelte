<script lang="ts">
    import {parseQuery} from "../location";
    import {onMount, onDestroy} from "svelte";
    import {getData, postData} from "../api";

    let voterInfo = {
        votingName: '',
        username: ''
    }
    let activePoll = null;
    let currentDecision = null;
    let errorMsg = '';
    let updateInterval = null;

    const parsedQuery = parseQuery();
    const votingId = parsedQuery.get('votingId');
    const voterKey = parsedQuery.get('voterKey');

    async function loadVoterInfo() {
        const response = await getData(`http://0.0.0.0:8000/api/votings/${votingId}/voters/info`, voterKey);
        if (response.ok) {
            voterInfo = response.data;
        } else {
            errorMsg = response.data.reason;
        }
    }

    if (votingId === undefined || voterKey === undefined) {
        location.hash = '#/not-found'
    } else {
        onMount(async () => {
            await loadVoterInfo();
            updateInterval = setInterval(async () => {
                const response = await getData(`http://0.0.0.0:8000/api/votings/${votingId}/polls/active`, voterKey)
                if (response.ok) {
                    if (response.data === null || activePoll === null || response.data.pollIndex !== activePoll.pollIndex) {
                        currentDecision = null;
                    }
                    activePoll = response.data;
                    console.log(activePoll)
                } else {
                    errorMsg = response.data.reason;
                }
            }, 1500)
        });
        onDestroy(() => {
            if (updateInterval) {
                clearInterval(updateInterval);
            }
        })
    }
</script>

<main>
    <div class="head">
        <h1>Voting:</h1>
        <h2>{voterInfo.votingName}</h2>
        <span>{voterInfo.username}</span>
    </div>
    {#if errorMsg !== ''}
        <div class="error-text">{errorMsg}. Please reload and try again</div>
    {/if}
    <div class="body">
        {#if activePoll === null}
            <div class="info-text">There is currently no active poll. Please wait till the administrator activates it.
            </div>
        {:else}
            <div class="poll">
                <h4 class="title">{activePoll.name}</h4>
                <span class="description">{activePoll.description}</span>
                <div class="flex-row">
                    <button class="flex-grow button-submit"
                            class:active={currentDecision === 'ACCEPT'}
                            on:click={() => currentDecision = 'ACCEPT'}
                    >Accept
                    </button>
                    <button class="flex-grow button-remove"
                            class:active={currentDecision === 'DECLINE'}
                            on:click={() => currentDecision = 'DECLINE'}
                    >Decline
                    </button>
                    <button class="flex-grow button-abstain"
                            class:active={currentDecision === 'ABSTAIN'}
                            on:click={() => currentDecision = 'ABSTAIN'}
                    >Abstain
                    </button>
                </div>
            </div>
            <button class="button send-vote-button">send vote</button>
        {/if}
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

    .poll .title {
        display: block;
        font-weight: bold;
        margin-top: 4px;
        margin-bottom: 4px;
    }

    .active {
        border-color: #333;
        border-width: 3px;
    }

    .poll .description {
        display: block;
    }

    .send-vote-button {
        width: 100%;
        margin-top: 40px;
    }
</style>