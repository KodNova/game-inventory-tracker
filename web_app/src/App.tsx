import Header from "./Header";
import RecentGames from "./RecentGames";

// App could also be called Discovery
export default function App() {
	return (
		<div className="bg-neutral-700 h-screen">
			<Header />
			<div className="flex flex-col gap-3">
				<p className="bg-amber-400 text-xl text-fuchsia-600">popular games</p>
				<RecentGames />
			</div>
		</div>
	);
}
