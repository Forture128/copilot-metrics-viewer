from typing import Any
from github.GithubObject import (
    Attribute,
    NonCompletableGithubObject,
    NotSet,
    _NotSetType,
)
from github.PaginatedList import PaginatedList

from datetime import datetime

import github.NamedUser
import github.Team


from github.Requester import Requester


class CopilotSeat(NonCompletableGithubObject):
    def _initAttributes(self) -> None:
        self._created_at: Attribute[datetime] | _NotSetType = NotSet
        self._updated_at: Attribute[datetime] | _NotSetType = NotSet
        self._pending_cancellation_date: Attribute[datetime] | _NotSetType = NotSet
        self._last_activity_at: Attribute[datetime] | _NotSetType = NotSet
        self._last_activity_editor: Attribute[str] | _NotSetType = NotSet
        self._plan_type: Attribute[str] | _NotSetType = NotSet
        self._assignee: Attribute[github.NamedUser.NamedUser] | _NotSetType = NotSet
        self._assigning_team: Attribute[github.Team.Team] | _NotSetType = NotSet

    def _useAttributes(self, attributes: dict[str, Any]) -> None:
        if "created_at" in attributes:
            self._created_at = self._makeDatetimeAttribute(attributes["created_at"])
        if "updated_at" in attributes:
            self._updated_at = self._makeDatetimeAttribute(attributes["updated_at"])
        if "pending_cancellation_date" in attributes:
            self._pending_cancellation_date = self._makeDatetimeAttribute(
                attributes["pending_cancellation_date"]
            )
        if "last_activity_at" in attributes:
            self._last_activity_at = self._makeDatetimeAttribute(
                attributes["last_activity_at"]
            )
        if "last_activity_editor" in attributes:
            self._last_activity_editor = self._makeStringAttribute(
                attributes["last_activity_editor"]
            )
        if "plan_type" in attributes:
            self._plan_type = self._makeStringAttribute(attributes["plan_type"])
        if "assignee" in attributes:
            self._assignee = self._makeClassAttribute(
                github.NamedUser.NamedUser, attributes["assignee"]
            )
        if "assigning_team" in attributes:
            self._assigning_team = self._makeClassAttribute(
                github.Team.Team, attributes["assigning_team"]
            )

    def __repr__(self) -> str:
        return self.get__repr__({"assignee": self._assignee.value})

    @property
    def created_at(self) -> datetime:
        return self._created_at.value

    @property
    def updated_at(self) -> datetime:
        return self._updated_at.value

    @property
    def pending_cancellation_date(self) -> datetime:
        return self._pending_cancellation_date.value

    @property
    def last_activity_at(self) -> datetime:
        return self._last_activity_at.value

    @property
    def last_activity_editor(self) -> str:
        return self._last_activity_editor.value

    @property
    def plan_type(self) -> str:
        return self._plan_type.value

    @property
    def assignee(self) -> github.NamedUser.NamedUser:
        return self._assignee.value

    @property
    def assigning_team(self) -> github.Team.Team:
        return self._assigning_team.value


class Copilot(NonCompletableGithubObject):
    def __init__(self, requester: Requester, org_name: str) -> None:
        super().__init__(requester, {}, {"org_name": org_name}, completed=True)

    def _initAttributes(self) -> None:
        self._org_name: Attribute[str] = NotSet

    def _useAttributes(self, attributes: dict[str, Any]) -> None:
        if "org_name" in attributes:  # pragma no branch
            self._org_name = self._makeStringAttribute(attributes["org_name"])

    def __repr__(self) -> str:
        return self.get__repr__(
            {
                "org_name": self._org_name.value
                if self._org_name is not NotSet
                else NotSet
            }
        )

    @property
    def org_name(self) -> str:
        return self._org_name.value

    def get_seats(self) -> PaginatedList[CopilotSeat]:
        """
        :calls: `GET /orgs/{org}/copilot/billing/seats <https://docs.github.com/en/rest/copilot/copilot-business>`_
        """
        url = f"/orgs/{self._org_name.value}/copilot/billing/seats"
        return PaginatedList(
            CopilotSeat,
            self._requester,
            url,
            None,
            list_item="seats",
        )

    def add_seats(self, selected_usernames: list[str]) -> int:
        """
        :calls: `POST /orgs/{org}/copilot/billing/selected_users <https://docs.github.com/en/rest/copilot/copilot-business>`_
        :param selected_usernames: List of usernames to add Copilot seats for
        :rtype: int
        :return: Number of seats created
        """
        url = f"/orgs/{self._org_name.value}/copilot/billing/selected_users"
        _, data = self._requester.requestJsonAndCheck(
            "POST",
            url,
            input={"selected_usernames": selected_usernames},
        )
        return data["seats_created"]

    def remove_seats(self, selected_usernames: list[str]) -> int:
        """
        :calls: `DELETE /orgs/{org}/copilot/billing/selected_users <https://docs.github.com/en/rest/copilot/copilot-business>`_
        :param selected_usernames: List of usernames to remove Copilot seats for
        :rtype: int
        :return: Number of seats cancelled
        """
        url = f"/orgs/{self._org_name.value}/copilot/billing/selected_users"
        _, data = self._requester.requestJsonAndCheck(
            "DELETE",
            url,
            input={"selected_usernames": selected_usernames},
        )
        return data["seats_cancelled"]
