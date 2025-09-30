<script setup lang="ts">
  import { ref } from 'vue';

  // Get
  const quote = ref();
  function getQuote()
  {
    fetch("/api").then((response) => {
      if (response.ok)
      {
        response.text().then((text) => {
          quote.value.innerText = text;
        });
      }
    });
  }
  getQuote();
  setInterval(getQuote, 2000);

  // Post
  let timeout: number;
  function resetPlaceholder()
  {
    clearTimeout(timeout);
    timeout = setTimeout(() => {
      input.value.placeholder = "Submit quote here!";
    }, 3000);
  }

  const input = ref();
  function postQuote()
  {
    if (input.value.value == "")
      return;
    
    input.value.placeholder = "Submitting...";

    fetch("/api/update", {
      method: "POST",
      body: input.value.value
    }).then((response) => {
      if (response.ok)
      {
        response.text().then((text) => {
          input.value.placeholder = text;
          resetPlaceholder();
        });
      }
      else
      {
        input.value.placeholder = "Error!";
        resetPlaceholder();
      }
    });

    input.value.value = "";
  }
</script>

<template>
  <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; gap:10px; height: 100dvh;">
    <p style="font-size: 20px;">What's the quote of 10 minutes?</p>
    <p style="font-size: 32px; text-align: center;" ref="quote"><strong>Retrieving quote...</strong></p>
    <div style="display: flex; gap: 5px;">
      <input style="border-radius: 7px; width: 100%; height: 23px;" placeholder="Submit quote here!" ref="input" @keyup.enter="postQuote">
      <input style="border-radius: 7px" type="button" value="Submit" @click="postQuote">
    </div>
  </div>
</template>

<style scoped></style>