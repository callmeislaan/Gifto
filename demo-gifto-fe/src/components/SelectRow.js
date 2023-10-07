import './InputRow.css';
import Select from 'react-select';

export default function SelectRow({ name, options, value, onChange }) {
    return (
        <table className="input-row">
            <tr>
                <td>
                    <span className='input-row-name'>{name}</span>
                </td>
                <td>
                    <Select
                            className="input"
                            classNamePrefix="select"
                            isDisabled={false}
                            isLoading={false}
                            isClearable={true}
                            isRtl={false}
                            isSearchable={true}
                            name="color"
                            options={options}
                            onChange={onChange}
                            value={value}
                        />
                </td>
            </tr>

        </table>
    )
}