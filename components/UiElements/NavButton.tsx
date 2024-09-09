import { Link, useNavigation, useRouter } from "expo-router";
import { Pressable, Text } from "react-native";
import React, { useRef } from "react";

interface NavButtonProps {
  path: string;
  pathName: string;
}

export default function NavButton({ path, pathName }: NavButtonProps) {
  return (
    <Link href={path} asChild>
      <Pressable>
        <Text>{pathName}</Text>
      </Pressable>
    </Link>
  );
}
