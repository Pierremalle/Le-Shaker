import ThemeProvider from "@/components/UiElements/theme/ThemeProvider";
import ModeClassiqueView from "@/components/UiElements/views/modeClassiqueView";

export default function modeClassique() {
  return (
    <ThemeProvider>
      <ModeClassiqueView />
    </ThemeProvider>
  );
}
