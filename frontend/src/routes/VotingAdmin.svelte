<script lang="ts">
    import {parseQuery} from "../location";
    import {getData} from "../api";
    import {onMount} from 'svelte';

    let voting = {
        votingId: "",
        name: "",
        voterCount: 0,
        polls: [],
        activePollIndex: null
    }

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

    if (votingId === undefined || adminKey === undefined) {
        location.hash = '#/not-found'
    } else {
        onMount(async () => {
            await loadVoting();
        });
    }

</script>

<main>
    <h1>Voting: {voting.name}</h1>
</main>

<style>
</style>