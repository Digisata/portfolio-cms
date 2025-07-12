// ExperienceTable.js
import React from "react";

const ExperienceTable = ({ experiences, handleEdit, handleDelete, error }) => {
  return (
    <div className="contain-table">
      {error && <p style={{ color: "red" }}>{error}</p>}
      <table className="striped-table">
        <thead>
          <tr>
            <th>No.</th>
            <th>Company</th>
            <th>Work Type</th>
            <th>Location</th>
            <th>Position</th>
            <th>Start Date</th>
            <th>End Date</th>
            <th>Is Present</th>
            <th colSpan={2} className="text-center">
              Actions
            </th>
          </tr>
        </thead>
        <tbody>
          {experiences.length > 0 ? (
            experiences.map((exp, i) => (
              <tr key={exp._id}>
                <td>{i + 1}</td>
                <td>{exp.company}</td>
                <td>{exp.work_type}</td>
                <td>{exp.location}</td>
                <td>{exp.position}</td>
                <td>{exp.start_date}</td>
                <td>{exp.end_date}</td>
                <td>{exp.is_present ? "✅" : "❌"}</td>
                <td className="text-right">
                  <button
                    onClick={() => handleEdit(exp._id)}
                    className="button muted-button"
                  >
                    Edit
                  </button>
                </td>
                <td className="text-left">
                  <button
                    onClick={() => handleDelete(exp._id)}
                    className="button muted-button"
                  >
                    Delete
                  </button>
                </td>
              </tr>
            ))
          ) : (
            <tr>
              <td colSpan={10}>No Experience Found</td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
};

export default ExperienceTable;
