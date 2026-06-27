import { createFileRoute } from "@tanstack/react-router";
import GameLibrary from "../GameLibrary";

export const Route = createFileRoute("/library")({
	component: RouteComponent,
});

function RouteComponent() {
	return <GameLibrary />;
}
