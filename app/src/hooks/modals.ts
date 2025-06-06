import PlayListModal from "../components/PlayListModal.vue";
import {openModal} from "jenesius-vue-modal";


export const createNewPlaylist = () => openModal(PlayListModal);