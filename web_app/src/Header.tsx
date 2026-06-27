import { Link } from "@tanstack/react-router";

export default function Header() {
	return (
		<div className="flex bg-neutral-500 gap-5 justify-center text-xl underline">
			<Link to="/">Discovery</Link>
			<Link to="/library">Library</Link>
		</div>
	);
}
