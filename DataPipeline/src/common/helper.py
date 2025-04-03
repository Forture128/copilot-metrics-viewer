from functools import wraps
import time
from functools import lru_cache
from typing import Any

from github import RateLimitExceededException


def rate_limit_handler(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        try:
            return func(*args, **kwargs)
        except RateLimitExceededException:
            # Wait and retry
            time.sleep(60)
            return func(*args, **kwargs)

    return wrapper


def cached_response(timeout: int = 300):
    def decorator(func):
        @lru_cache(maxsize=100)
        def wrapper(*args, **kwargs) -> Any:
            return func(*args, **kwargs)

        return wrapper

    return decorator
