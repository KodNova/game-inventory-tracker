import Header from "./Header";

// App could also be called Discovery
export default function App() {
	return (
		<div className="bg-neutral-700 h-screen">
			<Header />
			<div className="flex flex-col gap-3">
				<p className="bg-amber-400 text-xl text-fuchsia-600">popular games</p>
				<p className="bg-amber-600 text-xl text-fuchsia-600">recent releases</p>
			</div>
		</div>
	);
}
