<script lang="ts">
  import { Router, Link, Route } from "svelte-navigator";
  import Projects from './routes/Projects.svelte';
  import Groups from './routes/Groups.svelte';
  import { QueryClient, QueryClientProvider, useQuery } from '@sveltestack/svelte-query';
  import { User, postSession } from "./api/session";

  let user: User;
  let loggedIn = false;
  async function login() {
    // TODO: check the session cookie on mount instead of tr
    user = await postSession("TODO");
    loggedIn = true;
    console.log(`logged in as ${user.email}`);
  }

  const queryClient = new QueryClient();
</script>



<QueryClientProvider client={queryClient}>
  <main>
    {#if loggedIn}
      <Router>
        <nav>
          <Link to="/">Home</Link>
          <Link to="projects">Projects</Link>
          <Link to="groups">Groups</Link>
        </nav>
        <div>
          <Route path="/">
            Home
          </Route>
          <Route path="projects" component={Projects} />
          <Route path="groups" component={Groups} />
        </div>
      </Router>
      {:else}
        <button on:click={login}>Login</button>
      {/if}
    </main>  
</QueryClientProvider>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
