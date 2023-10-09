import './InputRow.css';

export default function InputDateRow({ name, value, onChange }) {
    return (
        <table className="input-row">
            <tr>
                <td>
                    <span className='input-row-name'>{name}</span>
                </td>
                <td>
                    <input className='input' type="date" onChange={(e) => onChange(e.target.value)} value={value}/>
                </td>
            </tr>

        </table>
    )
}