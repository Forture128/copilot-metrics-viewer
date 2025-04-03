class OrganizationServiceException(Exception):
    """Exception raised for errors in the OrganizationService class."""

    def __init__(self, message: str):
        # add format for each exception message
        self.message = f"[OrganizationServiceException]: {message}"
        super().__init__(self.message)


class GithubClientException(Exception):
    """Exception raised for errors in the GithubClient class."""

    def __init__(self, message: str):
        self.message = f"[GithubClientException]: {message}"
        super().__init__(self.message)
