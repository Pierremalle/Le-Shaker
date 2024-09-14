import ModeList from "./ModeList";
import ThemeProvider from "@/components/UiElements/theme/ThemeProvider";

export default function HomeScreen() {
  return (
    <ThemeProvider>
      <ModeList />
    </ThemeProvider>
  );
}
