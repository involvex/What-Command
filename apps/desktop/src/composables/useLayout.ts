import { ref, onMounted, onUnmounted } from "vue";

const MOBILE_BREAKPOINT = 768;

export function useLayout() {
  const isMobile = ref(
    typeof window !== "undefined" ? window.innerWidth < MOBILE_BREAKPOINT : false,
  );

  function update() {
    isMobile.value = window.innerWidth < MOBILE_BREAKPOINT;
  }

  onMounted(() => window.addEventListener("resize", update));
  onUnmounted(() => window.removeEventListener("resize", update));

  return { isMobile };
}
