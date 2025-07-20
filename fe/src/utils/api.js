// utils/api.js

import { jwtDecode } from "jwt-decode";

const BASE_URL = process.env.REACT_APP_BASE_URL || "http://localhost:8000";
const HOMEPAGE_URL = process.env.REACT_APP_BASE_URL || "http://localhost:3000";

export async function apiRequest(
  endpoint,
  method = "GET",
  body = null,
  token = null,
) {
  const headers = {
    "Content-Type": "application/json",
    Accept: "application/json",
  };

  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  const config = { method, headers };

  if (body) config.body = JSON.stringify(body);

  const response = await fetch(`${BASE_URL}/api${endpoint}`, config);

  if (response.status === 401) {
    // Unauthorized: clear auth state and redirect
    localStorage.removeItem("token");
    localStorage.setItem("is_authenticated", "false");
    window.location.href = HOMEPAGE_URL;
    return; // Exit early
  }

  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || "API request failed");
  }

  return await response.json();
}

export const login = (payload, token) =>
  apiRequest("/login", "POST", payload, token);

export const getCustomerDetail = (id, token = null) =>
  apiRequest(`/customer/${id}`, "GET", null, token);

export const updateCustomerById = (id, data, token) =>
  apiRequest(`/customer/${id}`, "PATCH", data, token);

export const getExperienceDetail = (id, token = null) =>
  apiRequest(`/experience/${id}`, "GET", null, token);

export const updateExperienceById = (id, data, token) =>
  apiRequest(`/experience/${id}`, "PATCH", data, token);

export const deleteExperienceById = (id, token) =>
  apiRequest(`/experience/${id}`, "DELETE", null, token);

export const addExperience = (payload, token) =>
  apiRequest("/experience", "POST", payload, token);

export const getProjectDetail = (id, token = null) =>
  apiRequest(`/project/${id}`, "GET", null, token);

export const updateProjectById = (id, data, token) =>
  apiRequest(`/project/${id}`, "PATCH", data, token);

export const deleteProjectById = (id, token) =>
  apiRequest(`/project/${id}`, "DELETE", null, token);

export const addProject = (payload, token) =>
  apiRequest("/project", "POST", payload, token);

export const getSkillDetail = (id, token = null) =>
  apiRequest(`/skill/${id}`, "GET", null, token);

export const updateSkillById = (id, data, token) =>
  apiRequest(`/skill/${id}`, "PATCH", data, token);

export const deleteSkillById = (id, token) =>
  apiRequest(`/skill/${id}`, "DELETE", null, token);

export const addSkill = (payload, token) =>
  apiRequest("/skill", "POST", payload, token);

export const getSocialDetail = (id, token = null) =>
  apiRequest(`/social/${id}`, "GET", null, token);

export const updateSocialById = (id, data, token) =>
  apiRequest(`/social/${id}`, "PATCH", data, token);

export const deleteSocialById = (id, token) =>
  apiRequest(`/social/${id}`, "DELETE", null, token);

export const addSocial = (payload, token) =>
  apiRequest("/social", "POST", payload, token);

export const getIdFromToken = () => {
  const token = localStorage.getItem("token");
  if (token) {
    const decoded = jwtDecode(token);
    return {
      id: decoded.sub,
      token,
    };
  }

  localStorage.removeItem("token");
  localStorage.setItem("is_authenticated", "false");
  window.location.href = HOMEPAGE_URL;
};
