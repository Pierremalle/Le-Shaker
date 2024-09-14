import ThemeProvider from "@/components/UiElements/theme/ThemeProvider";
import HomeView from "@/components/UiElements/views/HomeView";
import { StyleSheet, View } from "react-native";

export default function HomeScreen() {
  return (
    <ThemeProvider>
      <HomeView></HomeView>
    </ThemeProvider>
  );
}
