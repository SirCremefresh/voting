<script lang="ts">
    import Home from './routes/Home.svelte';
    import NotFound from './routes/NotFound.svelte';
    import VotingAdmin from './routes/VotingAdmin.svelte';
    import VotingVoter from './routes/VotingVoter.svelte';


    let page = loadPagerForRoute();
    window.addEventListener('hashchange', () => {
        page = loadPagerForRoute();
    })

    function loadPagerForRoute() {
        const hash = window.location
            .hash
            .trim()
            .replace(/^(#\/)/, "")
            .replace(/\?.*/, "");


        if (hash === '') {
            return Home;
        } else if (hash === 'voting/admin') {
            return VotingAdmin;
        } else if (hash === 'voting/voter') {
            return VotingVoter;
        }
        return NotFound;
    }
</script>

<main>
    <svelte:component this={page}/>
</main>

<style>
    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
</style>