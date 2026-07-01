import { useQuery } from "@tanstack/react-query";
import Header from "./Header";
const user_token = "test";

// type Game = {
// 	rawg_id: number;
// 	name: string;
// 	released: string | null;
// 	background_image: string | null;
// };

type Game = {
	rawg_id: number;
	name: string;
	user_token: string;
	background_image: string | null;
};

async function getGameLibrary(): Promise<Game[]> {
	const res = await fetch(
		`${import.meta.env.VITE_BACKEND_URL}/get_user_games/${user_token}`,
	);

	if (!res.ok) throw new Error("Failed to fetch recent games");

	const data = await res.json();

	return data.user_games;
}

export default function GameLibrary() {
	const {
		data: games = [],
		isLoading,
		error,
	} = useQuery({
		queryKey: ["userLibrary", user_token],
		queryFn: getGameLibrary,
	});

	if (isLoading) return <p>Loading...</p>;
	if (error) return <p>Failed to load games.</p>;

	return (
		<div className="bg-neutral-700 h-screen">
			<Header />
			<div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
				{games.map((game) => (
					<GameCard key={game.rawg_id} {...game} />
				))}
			</div>
		</div>
	);
}

function GameCard(props: Game) {
	return (
		<div className="flex flex-col border justify-center items-center gap-1 p-2 border-neutral-600">
			<img
				alt="background_image"
				src={props.background_image ?? undefined}
				className="w-24"
			/>
			<p>{props.name}</p>
			{/* <p>{props.released}</p> */}
		</div>
	);
}
