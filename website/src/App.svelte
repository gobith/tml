<script>
  import * as L from "leaflet-gpx";
  import "leaflet/dist/leaflet.css";
  import { onMount } from "svelte";

  const list = [
    { id: "5", name: "5 km", path: "./TML-5-km-AU-certifieerde-2021-2025.gpx" },
    {
      id: "10",
      name: "10 km",
      path: "./TML-10-km-AU-certifieerde-2021-2025.gpx",
    },
    {
      id: "16",
      name: "16,1 km",
      path: "./TML-161-km-AU-certifieerde-2021-2025.gpx",
    },
    {
      id: "21",
      name: "21,1 km",
      path: "./TML-211-km-AU-certifieerde-2021-2025.gpx",
    },
    { id: "test", name: "test", path: "./test.gpx" },
  ];

  let selection = list[3];

  let gpx;

  const changeGpx = (sel) => {
    if (!map) return;
    if (gpx) {
      gpx.remove();
    }
    gpx = new L.GPX(selection.path, { async: true }).on("loaded", function (e) {
      map.fitBounds(e.target.getBounds());
    });

    gpx.addTo(map);
  };

  $: changeGpx(selection);

  const tileUrl = () => {
    return `https://service.pdok.nl/brt/achtergrondkaart/wmts/v2_0/pastel/EPSG:3857/{z}/{x}/{y}.png`;
  };

  const tileLayer = L.tileLayer(tileUrl(), {
    maxZoom: 20,
  });

  const service = {
    data: { lat: 52.15517, lng: 5.3872, zoom: 8 },
  };

  let map_element;
  let map;

  const setup = () => {
    map = L.map(map_element, { zoomControl: false }).setView(
      [service.data.lat, service.data.lng],
      service.data.zoom
    );

    // tileLayer.addTo(map);

    L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
      attribution:
        '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
    }).addTo(map);

    changeGpx(selection);
  };

  onMount(setup);

  const select = (item) => {
    selection = item;
    console.log(item);
  };
</script>

<div class="container">
  <div class="list">
    <ul>
      {#each list as item}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <li
          class:active={selection.id === item.id}
          on:click={() => {
            select(item);
          }}
        >
          {item.name}
        </li>
      {/each}
    </ul>
  </div>

  <div class="map" id="mapid" bind:this={map_element} />
</div>

<style>
  .container {
    position: relative;
  }
  .map {
    height: 100vh;
    width: 100vw;
  }

  .list {
    position: absolute;
    top: 0;
    left: 0;
    background: transparent;
    padding: 1rem;
    z-index: 1000;
  }

  ul {
    list-style: none;
    padding: 0;
    margin: 0;
    /* background-color: rgba(0, 0, 0, 0.1); */
    /* border-radius: 50%;
    width: 200px;
    height: 200px; */
    text-align: center;
  }

  li {
    cursor: pointer;
    padding: 0.5rem;
  }

  li.active {
    background-color: rgba(0, 0, 0, 0.6);
    color: #fff;
    border-radius: 10px;
  }
</style>
