import { Button } from "react-native-paper";
import React from "react";
import { Link } from "expo-router";

interface NavButtonProps {
  path: string;
  pathName: string;
}

export default function NavButton({ path, pathName }: NavButtonProps) {
  return (
    <Link href={path}>
      <Button mode="contained">{pathName}</Button>
    </Link>
  );
}
