<script lang="ts">
    import { onMount } from 'svelte';
    export let label: string = null;
    export let name: string;
    export let placeholder: string;
    export let id: string;
    export let data: string[];
    export let value: string;

    let activeIndex = 0;
    let matches = [];
    let selected = false;
    let is_active = false;

    let dropdownMenuNode;
    let is_error = false;

    function handleKeyPress(event: KeyboardEvent) {
        switch (event.key) {
            case "Enter":
                if (is_active && matches.length) {
                    value = matches[activeIndex];
                    selected = true;
                    is_active = false;
                }
                break;
            case "Tab":
                if (is_active && matches.length) {
                    value = matches[activeIndex];
                    selected = true;
                }
                is_active = false;
                break;
            case "Escape":
                is_active = false;
                break;
            case "ArrowUp":
                if(!is_active) {
                    let index = data.indexOf(value);
                    value = index > 1
                        ? data[index - 1]
                        : data[0];
                } else {
                    activeIndex = activeIndex >= 1
                        ? activeIndex - 1
                        : 0;
                    value = matches[activeIndex];
                }
                selected = true;
                break;
            case "ArrowDown":
                if(!is_active) {
                    let index = data.indexOf(value);
                    value = index < data.length - 1
                        ? data[index + 1]
                        : data[data.length - 1];
                } else {
                    activeIndex = activeIndex < matches.length - 1
                        ? activeIndex + 1
                        : matches.length - 1;
                    value = matches[activeIndex];
                }
                selected = true;
                break;
            default:
                break;
        }
    }
    function handleClick(event:Event) {
        if(is_active) {
            is_active = false;
        } else if(selected) {
            matches = data;
            is_active = true;
        } else {
            value = (<HTMLInputElement>event.target).value;
            if (value.length >= 1) {
                matches = data.filter(
                    (item) =>
                        item.toUpperCase().indexOf(value.toUpperCase()) >= 0
                );
                is_active = matches.length > 0;
            } else {
                matches = data;
                is_active = true;
            }
        }

    }

    function handleBlur(event: Event) {
        is_error = value && value.length > 0 && data.indexOf(value) < 0;
        let relatedTarget = (<MouseEvent>event).relatedTarget;
        if (!dropdownMenuNode.contains(<Node>relatedTarget)) {
            matches = [];
            is_active = false;
        }
    }
    function handleFocus(event: Event) {
        (<HTMLInputElement>event.target).select()
    }

    function handleSelection(event: Event, selection: string) {
        activeIndex = 0;
        value = selection;
        matches = [];
        selected = true;
        is_active = false;
        is_error = false;
    }
    function updateQuery(e: Event) {
        if (selected && (<InputEvent>e).inputType === "deleteContentBackward") {
            selected = false;
                value = "";
                matches = [];
                is_active = false;
        } else {
            value = (<HTMLInputElement>e.target).value;
            if (value.length >= 1) {
                matches = data.filter(
                    (item) =>
                        item.toUpperCase().indexOf(value.toUpperCase()) >= 0
                );
                is_active = matches.length > 0;
            } else {
                matches = [];
                is_active = false;
            }
        }
    }

</script>

<div class="field">
    {#if label}
        <label for={id} class="label">{label}</label>
    {/if}
    <div class="control" id={`${id}-control`}>
        <div class="dropdown" class:is-active={is_active} style="display: block;">
            <div class="dropdown-trigger">
                <span class="control has-icons-right">
                    <input
                        type="text"
                        class="input is-small"
                        class:is-danger={is_error}
                        {id}
                        {name}
                        {value}
                        on:input={updateQuery}
                        on:keydown={handleKeyPress}
                        on:click={handleClick}
                        on:blur={handleBlur}
                        on:focus={handleFocus}
                        {placeholder}
                    />
                    <a href="/" class="icon is-right is-small" tabindex="-1">
                        <span class="fas fa-angle-down" />
                    </a>
                </span>
            </div>
            <div bind:this={dropdownMenuNode} class="dropdown-menu pt-0">
                {#if matches.length > 0}
                    <div class="dropdown-content pt-0 is-clipped">
                        {#each matches as match, index (match)}
                            <!-- svelte-ignore a11y-invalid-attribute -->
                            <a
                                class="dropdown-item"
                                class:is-active={index === activeIndex}
                                href="javascript:;"
                                on:click|preventDefault={(e) =>
                                    handleSelection(e, match)}
                            >
                                {match}
                            </a>
                        {/each}
                    </div>
                {/if}
            </div>
        </div>
    </div>
</div>
