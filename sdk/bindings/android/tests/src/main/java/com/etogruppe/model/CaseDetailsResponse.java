package com.etogruppe.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;

import com.etogruppe.deserializer.CaseDetailsResponseDeserializer;

@JsonDeserialize(using = CaseDetailsResponseDeserializer.class)
public class CaseDetailsResponse {

    @JsonProperty("case_id")
    public String caseId;
    @JsonProperty("archived")
    public boolean archived;
    @JsonProperty("status")
    public String status;

    public CaseDetailsResponse(String caseId, boolean archived, String status) {
        this.caseId = caseId;
        this.archived = archived;
        this.status = status;
    }

}
