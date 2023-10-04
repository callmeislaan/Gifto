import './Button.css';

export default function Button({value, onClick}) {
    return (
        <button onClick={onClick}>{value}</button>
    )
}