import { ref } from "vue";

const show = ref(false);
const component = ref();

export interface AppModalInterface {
  title: string;
  kind: "error" | "warning" | "success" | "info" | "secondary" | "neutral";
  message: string;
}
export const useModal = (config: AppModalInterface) => ({
  component,
  show,
  showModal: () => (show.value = true),
  hideModal: () => (show.value = false),
  title: config.title,
  message: config.message,
  kind: config.kind,
});
