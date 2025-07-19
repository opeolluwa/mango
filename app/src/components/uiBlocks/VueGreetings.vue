<template>
  <div>{{ greeting }}</div>
</template>

<script setup>
import { ref, onMounted, defineProps } from "vue";

// Props definition
const props = defineProps({
  username: {
    type: String,
    required: false,
    default: "",
  },
});

// Reactive greeting message
const greeting = ref("");

// Utility function to get random item from array
const getRandomItem = (array) => {
  const index = Math.floor(Math.random() * array.length);
  return array[index];
};

// Greeting options collection
const greetingOptions = {
  monday: [
    "Hello Monday!",
    "Monday is here!",
    "Show some Monday love",
    "Another fresh start",
  ],
  tuesday: ["Hello Tuesday!", "Tuesday vibes", "Keep pushing forward"],
  wednesday: ["Welcome Back", "Midweek motivation", "Hump day!"],
  thursday: [
    "#TBT",
    "Throwback Thursday😋",
    "Time for throwback",
    "Let's do some throwback",
    "Hectic week, huh? ",
  ],
  friday: [
    "#TGIF",
    "Weekend is here!",
    "Have a nice weekend",
    "How has the week been?",
    "Thank God It's Friday",
    "It's weekend",
    "Hello",
    "Weekend is near",
  ],
  saturday: [
    "You deserve some break",
    "Take timeout, enjoy",
    "Hangout! It's Saturday",
    "Still indoor? Hangout!",
    "You deserve a great time",
    "Hey! It's Saturday",
  ],
  sunday: [
    "Happy New Week!",
    "Happy Sunday!",
    "It's a new week💃💃",
    "New week wishes",
  ],
  morning: [
    "Blessed morning",
    "Good Morning",
    "Trust you slept well",
    "Beautiful morning",
    "A new day",
    "Have a fruitful day",
  ],
  dayBreak: [
    "How is your day going",
    "How has the day been",
    "A fresh morning, huh?",
    "How's the weather?",
    "Howdy?",
  ],
  afternoon: [
    "Good afternoon!",
    "How's the weather outside",
    "How is your day going",
    "How has work been?",
    "How has your day been?",
    "How is your day going",
    "How has the day been",
    "How's the weather?",
    "Howdy?",
  ],
  evening: ["Good evening", "How was your day", "How did your day go"],
  midnight: [
    "You up so late?",
    "You should probably be in bed",
    "You should probably be in bed by now",
    "Surprised to see you up so late",
    "Up so early, huh?",
    "Up so early?",
    "Working Late",
    "Hi there!",
    "Aren't you sleeping?",
  ],
  newMonth: [
    "Happy new month!",
    "Blessed new Month",
    "Have a Happy new month!",
    "It's a new month!",
  ],
  newYear: ["Happy new year", "Happy Holidays"],
  christmas: ["Merry Christmas", "Season greetings"],
  valentine: ["It's Valentine", "Happy Valentine"],
  other: ["Holla!", "Hello!", "Welcome", "Howdy?", "Ciao!"],
};

// Generate greeting based on current date and time
const generateGreeting = () => {
  const now = new Date();
  const day = now.getDay();
  const hour = now.getHours();
  const date = now.getDate();
  const month = now.getMonth();

  const isNewMonth = date === 1;
  const isChristmas = month === 11 && date === 25;
  const isValentine = month === 1 && date === 14;
  const isNewYear = month === 0 && date === 1;

  let greetingParts = {
    time: "",
    day: "",
    occasion: "",
    other: "",
  };

  // Handle day-specific greetings
  switch (day) {
    case 0: // Sunday
      greetingParts.day = getRandomItem(greetingOptions.sunday);
      break;
    case 1: // Monday
      greetingParts.day = getRandomItem(greetingOptions.monday);
      break;
    case 2: // Tuesday
      greetingParts.day = getRandomItem(greetingOptions.tuesday);
      break;
    case 3: // Wednesday
      greetingParts.day = getRandomItem(greetingOptions.wednesday);
      break;
    case 4: // Thursday
      greetingParts.day = getRandomItem(greetingOptions.thursday);
      break;
    case 5: // Friday
      greetingParts.day = getRandomItem(greetingOptions.friday);
      break;
    case 6: // Saturday
      greetingParts.day = getRandomItem(greetingOptions.saturday);
      break;
  }

  // Handle time-specific greetings
  if (hour >= 6 && hour < 8) {
    greetingParts.time = getRandomItem(greetingOptions.morning);
  } else if (hour >= 8 && hour < 10) {
    greetingParts.time = getRandomItem(greetingOptions.dayBreak);
  } else if (hour >= 12 && hour < 16) {
    greetingParts.time = getRandomItem(greetingOptions.afternoon);
  } else if (hour >= 16 && hour <= 20) {
    greetingParts.time = getRandomItem(greetingOptions.evening);
  } else if (hour >= 1 && hour < 6) {
    greetingParts.time = getRandomItem(greetingOptions.midnight);
  }

  // Handle special occasions
  if (isNewYear) {
    greetingParts.occasion = getRandomItem(greetingOptions.newYear);
  } else if (isNewMonth) {
    greetingParts.occasion = getRandomItem(greetingOptions.newMonth);
  } else if (isChristmas) {
    greetingParts.occasion = getRandomItem(greetingOptions.christmas);
  } else if (isValentine) {
    greetingParts.occasion = getRandomItem(greetingOptions.valentine);
  }

  // Fallback to other greetings
  greetingParts.other = getRandomItem(greetingOptions.other);

  // Priority: occasion > time > day > other
  const finalGreeting =
    greetingParts.occasion ||
    greetingParts.time ||
    greetingParts.day ||
    greetingParts.other;

  // Add username if provided
  if (props.username) {
    return `${finalGreeting} ${props.username}!`;
  }

  return finalGreeting;
};

// Initialize greeting on component mount
onMounted(() => {
  greeting.value = generateGreeting();
  console.log("Generated greeting:", greeting.value);
});

// Expose method to regenerate greeting (optional)
const refreshGreeting = () => {
  greeting.value = generateGreeting();
};

// Export for potential external use
defineExpose({
  refreshGreeting,
  greeting,
});
</script>

<style scoped>
.vue__greetings {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  font-size: 1.2rem;
  font-weight: 500;
  color: #2c3e50;
  padding: 1rem;
  text-align: center;
  border-radius: 8px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-color: #f8f9fa;
}
</style>
