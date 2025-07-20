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

// Greeting options collection - Reimagined with modern, diverse, and engaging content
const greetingOptions = {
  monday: [
    "Rise and shine, it's Monday! ✨",
    "New week, new possibilities 🌟",
    "Monday motivation activated 💪",
    "Let's make this Monday count! 🚀",
    "Fresh start, fresh energy ⚡",
    "Monday vibes incoming 🌅",
    "Ready to conquer the week? 🎯",
    "Monday: where dreams meet action 💫"
  ],
  tuesday: [
    "Tuesday energy is flowing! 🌊",
    "Tuesday: the real work begins 💼",
    "Keep that momentum going! 🔥",
    "Tuesday vibes are underrated ✨",
    "Midweek prep mode activated 🎯",
    "Tuesday: your productivity peak 📈",
    "Let's make Tuesday terrific! 🌟",
    "Tuesday: where magic happens ✨"
  ],
  wednesday: [
    "Happy Hump Day! 🐪",
    "Wednesday wisdom: we're halfway there! 🎯",
    "Midweek motivation check-in 💪",
    "Wednesday: the sweet spot of the week 🍯",
    "Over the hump and cruising! 🚗",
    "Wednesday vibes are everything ✨",
    "Midweek magic in progress 🌟",
    "Wednesday: where balance meets brilliance ⚖️"
  ],
  thursday: [
    "Throwback Thursday vibes! 📸",
    "Thursday: the new Friday energy 🎉",
    "Almost there, keep pushing! 💪",
    "Thursday thoughts: weekend prep mode 🎯",
    "Thursday: where anticipation meets action ⚡",
    "One more day to greatness! 🌟",
    "Thursday: your pre-weekend power hour ⏰",
    "Thursday: the calm before the weekend storm 🌅"
  ],
  friday: [
    "TGIF! Let's celebrate! 🎉",
    "Friday feeling: pure joy! ✨",
    "Weekend mode: activated! 🚀",
    "Friday: where work meets weekend! 🎯",
    "Cheers to the weekend! 🥂",
    "Friday: your reward for a week well done! 🏆",
    "Weekend vibes incoming! 🌊",
    "Friday: the gateway to freedom! 🗝️"
  ],
  saturday: [
    "Saturday: your day to shine! ✨",
    "Weekend warrior mode: activated! 🛡️",
    "Saturday: where relaxation meets adventure! 🌟",
    "Time to recharge and refresh! 🔋",
    "Saturday: your personal paradise! 🏝️",
    "Weekend magic in full effect! ✨",
    "Saturday: where memories are made! 📸",
    "Your weekend, your rules! 👑"
  ],
  sunday: [
    "Sunday serenity! 🧘‍♀️",
    "Sunday: your soul's reset button! 🔄",
    "New week, new you! 🌟",
    "Sunday: where peace meets preparation! ✨",
    "Ready for a fresh start? 🚀",
    "Sunday: your weekly wellness check! 💚",
    "New week wishes coming your way! 🌈",
    "Sunday: the calm before the storm! 🌅"
  ],
  morning: [
    "Good morning, beautiful soul! ✨",
    "Rise and shine, it's a new day! 🌅",
    "Morning magic is in the air! ✨",
    "Good morning, world! 🌍",
    "Fresh day, fresh perspective! 🌟",
    "Morning vibes: pure positivity! 💫",
    "Good morning, let's make it count! 🎯",
    "Morning: where dreams wake up! 💭"
  ],
  dayBreak: [
    "How's your morning treating you? ☀️",
    "Morning check-in: how are you feeling? 💭",
    "Fresh morning, fresh opportunities! 🌟",
    "Morning vibes check: positive? ✨",
    "How's the world looking this morning? 🌍",
    "Morning energy: flowing or flowing? 🌊",
    "Good morning! Ready to rock this day? 🚀",
    "Morning thoughts: what's on your mind? 💭"
  ],
  afternoon: [
    "Good afternoon, sunshine! ☀️",
    "Afternoon energy: still going strong? 💪",
    "How's your day unfolding? 🌟",
    "Afternoon check-in: feeling productive? 📈",
    "Good afternoon! How's the weather treating you? 🌤️",
    "Afternoon vibes: smooth sailing? ⛵",
    "How's your day been so far? ✨",
    "Afternoon: where momentum meets magic! 🌟"
  ],
  evening: [
    "Good evening, beautiful! ✨",
    "Evening vibes: winding down or powering up? 🌙",
    "How was your day? Ready to share? 💭",
    "Good evening! How did today treat you? 🌟",
    "Evening check-in: mission accomplished? 🎯",
    "Evening: where reflection meets relaxation! 🧘‍♀️",
    "How's your evening shaping up? ✨",
    "Good evening! Time to unwind? 🌅"
  ],
  midnight: [
    "Late night thoughts? 🌙",
    "Night owl or early bird? 🦉",
    "Working late or living life? 💻",
    "Midnight musings: what's on your mind? 💭",
    "Late night energy: still going strong? ⚡",
    "Night owl check-in: how's the night treating you? 🌙",
    "Late night vibes: productive or peaceful? ✨",
    "Midnight: where creativity meets quiet! 🌟"
  ],
  newMonth: [
    "Happy new month! Fresh start ahead! 🌟",
    "New month, new goals, new you! 🎯",
    "Welcome to a brand new month! ✨",
    "New month vibes: ready to conquer? 💪",
    "Happy new month! What's your focus? 🎯",
    "New month: where dreams get a fresh start! 🌅",
    "Happy new month! Time for new adventures! 🚀",
    "New month: your canvas for greatness! 🎨"
  ],
  newYear: [
    "Happy New Year! Here's to new beginnings! 🎉",
    "New Year, new possibilities! ✨",
    "Happy New Year! Ready for the journey ahead? 🚀",
    "New Year vibes: pure excitement! 🌟",
    "Happy New Year! What's your vision? 👁️",
    "New Year: where dreams become reality! 💫",
    "Happy New Year! Time to shine brighter! ✨",
    "New Year: your year of transformation! 🦋"
  ],
  christmas: [
    "Merry Christmas! Joy to the world! 🎄",
    "Christmas magic is in the air! ✨",
    "Merry Christmas! Peace and love! 💚",
    "Christmas vibes: pure holiday spirit! 🎁",
    "Merry Christmas! Season's greetings! 🌟",
    "Christmas: where joy meets celebration! 🎉",
    "Merry Christmas! Warmest wishes! 🔥",
    "Christmas: the most wonderful time! ⭐"
  ],
  valentine: [
    "Happy Valentine's Day! Love is in the air! 💕",
    "Valentine vibes: spreading love! ✨",
    "Happy Valentine's Day! Heart full of love! 💖",
    "Valentine's Day: where love takes center stage! 🌹",
    "Happy Valentine's Day! Love yourself first! 💝",
    "Valentine vibes: pure romance! 💕",
    "Happy Valentine's Day! Love wins! 🏆",
    "Valentine's Day: celebrating all forms of love! 💫"
  ],
  other: [
    "Hello there, beautiful human! ✨",
    "Hey! How's life treating you? 🌟",
    "Greetings from the digital realm! 🌐",
    "Hello! Ready to make today amazing? 🚀",
    "Hey there! What's your story? 📖",
    "Hello! How can I brighten your day? ☀️",
    "Greetings! What's on your mind? 💭",
    "Hello! Let's make some magic happen! ✨"
  ]
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
