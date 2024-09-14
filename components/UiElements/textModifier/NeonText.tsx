import React from "react";
import { StyleSheet } from "react-native";
import { useTheme } from "react-native-paper";
import { Text } from "react-native-paper";
import { VariantProp } from "react-native-paper/lib/typescript/components/Typography/types";

interface NeonTextProps {
  txt: string;
  size: VariantProp<string>;
}

export default function NeonText({ txt, size }: NeonTextProps) {
  const { colors } = useTheme(); // Déplacez l'appel de useTheme ici

  return (
    <Text
      style={[
        styles.neonText,
        { color: colors.yellow, textShadowColor: colors.yellowContainer },
      ]}
      variant={size}
    >
      {txt}
    </Text>
  );
}

const styles = StyleSheet.create({
  neonText: {
    fontSize: 60,
    fontWeight: "bold",

    textShadowOffset: { width: -1, height: 1 },
    textShadowRadius: 50,
  },
});
