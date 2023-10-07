import './InputRow.css';

export default function InputRow({ name, value, onChange }) {
    return (
        <table className="input-row">
            <tr>
                <td>
                    <span className='input-row-name'>{name}</span>
                </td>
                <td>
                    <input className='input' type="text" onChange={(e) => onChange(e.target.value)} value={value}/>
                </td>
            </tr>

        </table>
    )
}