<script lang="ts">
    import Pie from "svelte-chartjs/src/Pie.svelte"

    const data = {
  labels: [
    'Red',
    'Blue',
    'Yellow'
  ],
  datasets: [{
    label: 'My First Dataset',
    data: [300, 50, 100],
    backgroundColor: [
      'rgb(255, 99, 132)',
      'rgb(54, 162, 235)',
      'rgb(255, 205, 86)'
    ],
    hoverOffset: 4
  }]
};
</script>

<div>
    <Pie data={data} />
</div>

<style>

</style>