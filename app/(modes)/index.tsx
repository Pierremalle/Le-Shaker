import ModeList from "@/components/UiElements/views/ModeListView";
import ThemeProvider from "@/components/UiElements/theme/ThemeProvider";

export default function HomeScreen() {
  return (
    <ThemeProvider>
      <ModeList />
    </ThemeProvider>
  );
}
