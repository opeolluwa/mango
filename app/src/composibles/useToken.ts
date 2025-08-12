import { Router } from 'vue-router';

export default function useToken(
  routeName: string,
  token: string | number,
  router: Router
) {
  router.push({ name: routeName, query: { token: token.toString() } });
}
