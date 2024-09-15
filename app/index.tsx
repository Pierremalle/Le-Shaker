import ThemeProvider from "@/components/UiElements/theme/ThemeProvider";
import HomeView from "@/components/UiElements/views/HomeView";

export default function HomeScreen() {
  return (
    <ThemeProvider>
      <HomeView></HomeView>
    </ThemeProvider>
  );
}
