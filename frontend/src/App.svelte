<script lang="ts">
  import { Router, Link, Route } from "svelte-navigator";
  import Projects from "./routes/Projects.svelte";
  import Groups from "./routes/Teams.svelte";
  import { QueryClient, QueryClientProvider, useQuery } from "@sveltestack/svelte-query";
  import { type User, postSession } from "./api/session";

  let user: User;
  let loggedIn = false;
  async function login() {
    // TODO: check the session cookie on mount instead of tr
    user = await postSession("TODO");
    loggedIn = true;
    console.log(`logged in as ${user.email}`);
  }

  function navProps(linkProps) {
    if (linkProps.isPartiallyCurrent) {
      return { class: "navbar-item is-active" };
    } else {
      return { class: "navbar-item" };
    }
  }
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        staleTime: 60 * 1000
      }
    }
  });
</script>

<QueryClientProvider client={queryClient}>
  <Router>
    <nav class="navbar is-light" role="navigation" aria-label="main navigation">
      <div class="container">
        <div class="navbar-brand">
          <Link to="/" class="navbar-item">
            <img src="/tardis-logo.png" alt="logo" />
            <span class="title">Capaldi</span>
          </Link>
        </div>

        <div id="navbarBasicExample" class="navbar-menu">
          {#if loggedIn}
            <div class="navbar-start">
              <Link to="teams" getProps={navProps}>Team</Link>
              <Link to="projects" getProps={navProps}>Projects</Link>
            </div>
          {/if}

          {#if loggedIn}
            <div class="navbar-end">
              <div class="navbar-item">
                <div class="buttons">
                  <a class="button is-light" href="/"><ion-icon name="log-out" /> &nbsp; Logout</a>
                </div>
              </div>
            </div>
          {/if}
        </div>
      </div>
    </nav>
    <section class="section">
      <main class="container">
        {#if loggedIn}
          <Route path="projects" component={Projects} />
          <Route path="teams"><Groups /></Route>
          <Route path="resources/:id">TODO</Route>
        {:else}
          <div class="column is-half is-offset-one-quarter">
            <form class="box" on:submit|preventDefault={login}>
              <div class="field">
                <label for="email" class="label">Email</label>
                <div class="control">
                  <input
                    id="email"
                    class="input"
                    type="email"
                    placeholder="e.g. alice@example.com"
                  />
                </div>
              </div>

              <button type="submit" class="button is-primary"
                ><ion-icon name="log-in" /> &nbsp; Sign in</button
              >
            </form>
          </div>
        {/if}
      </main>
    </section>
  </Router>
</QueryClientProvider>

<style>
</style>
