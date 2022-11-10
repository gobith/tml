<script lang="ts">
  import * as L from "leaflet";
  import "leaflet/dist/leaflet.css";
  import { onMount } from "svelte";

  // type ListItem = {
  //   id: string;
  //   name: string;
  //   path: string;
  // };

  const list = [
    { id: "5", name: "5 km", path: "./5km.json", data: null, layerGroup: null },
    {
      id: "10",
      name: "10 km",
      path: "./10km.json",
      data: null,
      layerGroup: null,
    },
    {
      id: "16",
      name: "16 km",
      path: "./16km.json",
      data: null,
      layerGroup: null,
    },
    {
      id: "21",
      name: "21 km",
      path: "./21km.json",
      data: null,
      layerGroup: null,
    },
  ];

  let polyline;
  let selection = list[0];
  let currentGroup = null;

  $: changeItem(selection);

  const changeItem = async (item) => {
    if (!item.data) {
      console.log("fetching data");
      let response = await fetch(item.path);
      item.data = await response.json();

      item.layerGroup = L.layerGroup();

      item.data.way_points.forEach((way_point) => {
        let myIcon = L.divIcon({
          html: `<div>${way_point.name}</div>`,
          iconSize: null,
          iconAnchor: [10, 10],
          className: "km-icon",
        });

        L.marker([way_point.lat, way_point.lon], { icon: myIcon }).addTo(
          item.layerGroup
        );
      });

      item.polyline = L.polyline(item.data.track_points, {
        color: "rgba(0,0,0,0.5)",
      });
      item.polyline.addTo(item.layerGroup);
    }

    if (currentGroup) {
      currentGroup.remove();
    }

    item.layerGroup.addTo(map);
    map.fitBounds(item.polyline.getBounds());
    currentGroup = item.layerGroup;
  };

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
  };

  onMount(setup);
</script>

<div class="container">
  <div class="list">
    <ul>
      {#each list as item}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <li
          class:active={selection.id === item.id}
          on:click={() => {
            selection = item;
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
