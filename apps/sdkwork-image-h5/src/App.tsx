import { AuthGate } from "./AuthGate";
import { bootstrap } from "./bootstrap/runtime";

bootstrap();

export function App() {
  return (
    <AuthGate>
      <div id="app-root" />
    </AuthGate>
  );
}
